import type { MjsConfig } from "../../main/domain/interfaces/MjsConfig.js";
import { DistEmptier } from "../../main/domain/services/DistEmptier.js";
import type { ExportsConfig } from "../../main/domain/valueObjects/ExportsConfig.js";
import type { FileNode } from "../../shared/domain/entities/FileNode.js";
import type { BuildOrchestrator } from "../../shared/domain/interfaces/BuildOrchestrator.js";
import type { FilesRepository } from "../../shared/domain/interfaces/FilesRepository.js";
import { ExtensionChanger } from "../../shared/domain/services/ExtensionChanger.js";
import { BuildOrchestratorResult } from "../../shared/domain/valueObjects/BuildOrchestratorResult.js";
import { PackageJsonExpectation } from "../../shared/domain/valueObjects/PackageJsonExpectation.js";
import { logger } from "../../shared/supporting/logger.js";
import { ModuleReferenceChanger } from "../domain/services/ModuleReferenceChanger.js";

export class ModuleBuildOrchestrator implements BuildOrchestrator {
	private readonly extensionChanger: ExtensionChanger;
	private readonly referenceChanger: ModuleReferenceChanger;

	constructor(
		private readonly filesRepository: FilesRepository,
		private readonly packageDir: FileNode,
		private readonly exportsConfig: ExportsConfig,
		private readonly mjsConfig: MjsConfig,
	) {
		this.extensionChanger = new ExtensionChanger(this.filesRepository);
		this.referenceChanger = new ModuleReferenceChanger(this.filesRepository);
	}

	async build(): Promise<BuildOrchestratorResult> {
		const startTime = Date.now();
		const builder = this.mjsConfig.getBuilder();

		await new DistEmptier(
			this.filesRepository,
			this.packageDir,
			builder.outdir,
		).remove();

		await builder.build(this.packageDir);
		await this.extensionChanger.changeInDir(builder.outdir, "js", "mjs");
		await this.referenceChanger.changeReferencesInDir(builder.outdir);

		const result = new BuildOrchestratorResult(
			await this.createPackageJsonExpectation(builder.outdir),
		);

		const endTime = Date.now();
		logger.debug(`Built ESM Module: ${endTime - startTime}ms`);

		return result;
	}

	private async createPackageJsonExpectation(
		outdir: string,
	): Promise<PackageJsonExpectation> {
		return new PackageJsonExpectation(this.filesRepository, {
			module: await this.generatePackageJsonMain(outdir),
			exports: await this.generatePackageJsonExports(outdir),
		});
	}

	private generatePackageJsonMain(outdir: string): Promise<string> {
		return this.distFromSrc(this.exportsConfig.getRootExport(), outdir);
	}

	private async distFromSrc(srcUri: string, outdir: string): Promise<string> {
		return srcUri
			.replace("./src", await this.packageDir.getRelativeUriOf(outdir))
			.replace(".ts", ".mjs");
	}

	private async generatePackageJsonExports(
		outdir: string,
	): Promise<Record<string, Record<"import", string>>> {
		const entries = this.exportsConfig
			.entries()
			.map(async ([k, v]) => [
				k,
				{ import: await this.distFromSrc(v, outdir) },
			]);

		return Object.fromEntries(await Promise.all(entries));
	}
}
