import type { EsmConfig } from "../../main/domain/interfaces/EsmConfig.js";
import type { DistEmptier } from "../../main/domain/services/DistEmptier.js";
import { TypeEnforcer } from "../../main/domain/services/TypeEnforcer.js";
import type { ExportsConfig } from "../../main/domain/valueObjects/ExportsConfig.js";
import type { FileNode } from "../../shared/domain/entities/FileNode.js";
import type { BuildOrchestrator } from "../../shared/domain/interfaces/BuildOrchestrator.js";
import type { ExtensionChanger } from "../../shared/domain/interfaces/ExtensionChanger.js";
import type { FilesRepository } from "../../shared/domain/interfaces/FilesRepository.js";
import { BuildOrchestratorResult } from "../../shared/domain/valueObjects/BuildOrchestratorResult.js";
import { logger } from "../../shared/supporting/logger.js";
import type { ModuleReferenceChanger } from "../domain/interfaces/ModuleReferenceChanger.js";
import { ModulePackageJsonExpectationFactory } from "../domain/services/ModulePackageJsonExpectationFactory.js";
import { ModuleOutDir } from "../domain/valueObjects/ModuleOutDir.js";

export class ModuleBuildOrchestrator implements BuildOrchestrator {
	private readonly outDir: ModuleOutDir;
	private readonly typeEnforcer: TypeEnforcer;
	private readonly packageJsonExpectationFactory: ModulePackageJsonExpectationFactory;

	constructor(
		private readonly filesRepository: FilesRepository,
		private readonly distEmptier: DistEmptier,
		private readonly extensionChanger: ExtensionChanger,
		private readonly referenceChanger: ModuleReferenceChanger,
		private readonly packageDir: FileNode,
		private readonly exportsConfig: ExportsConfig,
		private readonly mjsConfig: EsmConfig,
		distDirUri: string,
	) {
		this.outDir = new ModuleOutDir(distDirUri);
		this.typeEnforcer = new TypeEnforcer(this.filesRepository);
		this.packageJsonExpectationFactory =
			new ModulePackageJsonExpectationFactory(
				this.filesRepository,
				this.exportsConfig,
				this.packageDir,
				this.outDir,
			);
	}

	async build(): Promise<BuildOrchestratorResult> {
		const startTime = Date.now();
		const builder = this.mjsConfig.getBuilder();
		await this.distEmptier.remove(this.outDir.uri);
		await builder.build(this.packageDir, this.outDir.uri);
		await this.referenceChanger.changeReferencesInDir(this.outDir.uri);
		await this.typeEnforcer.enforce("module", this.outDir);

		const packageJsonExpectation =
			await this.packageJsonExpectationFactory.create();

		const result = new BuildOrchestratorResult(packageJsonExpectation);

		const endTime = Date.now();
		logger.debug(`Built ESM Module: ${endTime - startTime}ms`);

		return result;
	}
}
