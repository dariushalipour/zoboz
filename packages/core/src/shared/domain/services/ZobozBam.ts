import { type ChildProcessByStdio, spawn } from "child_process";
import { createRequire } from "module";
import type { Readable, Writable } from "stream";
import { execPath } from "process";
import { DealBreakerError } from "../errors/DealBreakerError";

export type SpecifiersReformatterConfig = {
	absoluteSourceDir: string;
	absoluteOutputDir: string;
	outputFormat: "dts" | "esm" | "cjs";
};

export type PackageJsonVerifierConfig = {
	canUpdatePackageJson: boolean;
};

type ZobozBamProcess = ChildProcessByStdio<Writable, Readable, Readable>;

export class ZobozBam {
	private readyProcess: null | Promise<ZobozBamProcess>;

	constructor(private readonly absolutePackageDir: string) {}

	async reformatSpecifiers(config: SpecifiersReformatterConfig): Promise<void> {
		await this.runCommand([
			"reformat-specifiers",
			"--absolute-package-dir",
			this.absolutePackageDir,
			"--absolute-source-dir",
			config.absoluteSourceDir,
			"--absolute-output-dir",
			config.absoluteOutputDir,
			"--output-format",
			config.outputFormat,
		]);
	}

	async verifyPackageJson(config: PackageJsonVerifierConfig): Promise<void> {
		await this.runCommand(
			[
				"verify-package-json",
				"--absolute-package-dir",
				this.absolutePackageDir,
				config.canUpdatePackageJson ? "--can-update-package-json" : "",
			].filter(Boolean),
		);
	}

	async exit(): Promise<void> {
		await this.runCommand(["exit"]);
		this.readyProcess = null;
	}

	private async runCommand(args: string[]): Promise<string> {
		const commandResult = withResolvers<string>();

		this.getReadyProcess().then((zobozBamProcess) => {
			const response = this.waitResponse(zobozBamProcess);
			zobozBamProcess.stdin.write(this.formatCommand(args));

			this.readyProcess = response
				.then(commandResult.resolve)
				.catch(commandResult.reject)
				.then(() => zobozBamProcess);
		});

		return commandResult.promise;
	}

	private formatCommand(args: string[]): string {
		return `${args.join(" ").replace(/\\/g, "\\\\")}\n`;
	}

	private async getReadyProcess(): Promise<ZobozBamProcess> {
		if (this.readyProcess) {
			return this.readyProcess;
		}

		const requireFunc =
			typeof require !== "undefined"
				? require
				: // @ts-ignore
					createRequire(import.meta.url);

		const zobozBamProcess = spawn(
			execPath,
			[requireFunc.resolve("@zoboz/bam")],
			{ stdio: ["pipe", "pipe", "pipe"] },
		);

		this.readyProcess = this.waitResponse(zobozBamProcess).then(
			() => zobozBamProcess,
		);

		return this.readyProcess;
	}

	private waitResponse(zobozBamProcess: ZobozBamProcess): Promise<string> {
		return new Promise<string>((resolve, reject) => {
			let errData = "";
			let outData = "";

			const onErrData = (chunk) => {
				errData += chunk.toString();
			};

			const onOutData = (chunk) => {
				outData += chunk.toString();
				// based on zoboz-bam, it means it's ready for the next command
				if (outData.endsWith("zoboz $ ")) {
					zobozBamProcess.stderr.removeListener("data", onErrData);
					zobozBamProcess.stdout.removeListener("data", onOutData);

					if (errData !== "") {
						reject(new DealBreakerError(errData));
					} else {
						resolve(outData);
					}
				}
			};

			zobozBamProcess.stderr.addListener("data", onErrData);
			zobozBamProcess.stdout.addListener("data", onOutData);
		});
	}
}

function withResolvers<T>(): {
	promise: Promise<T>;
	resolve: (data: T) => void;
	reject: (error: Error) => void;
} {
	let resolve: (data: T) => void;
	let reject: (error: Error) => void;

	const promise = new Promise<T>((res, rej) => {
		resolve = res;
		reject = rej;
	});

	// @ts-ignore
	return { promise, resolve, reject };
}
