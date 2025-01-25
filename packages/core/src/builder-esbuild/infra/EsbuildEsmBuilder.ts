import type {
	BuildParams,
	Builder,
} from "@shared/domain/interfaces/Builder.js";
import * as esbuild from "esbuild";
import * as process from "process";
import type { EsbuildOptions } from "../domain/interfaces/EsbuildOptions.js";

export class EsbuildEsmBuilder implements Builder {
	constructor(private readonly buildOptions?: EsbuildOptions) {}

	async build({ srcDir, outDir, logger }: BuildParams): Promise<void> {
		logger.pending(`Building ES Module by esbuild to ${outDir.uri}`);

		await esbuild.build({
			absWorkingDir: process.cwd(),
			entryPoints: [`./${srcDir.uri}/**/*.ts`, `./${srcDir.uri}/**/*.tsx`],
			outdir: outDir.absoluteUri,
			format: "esm",
			platform: "node",
			...(this.buildOptions || {}),
		});
	}
}
