import { DealBreakerError } from "@shared/domain/errors/DealBreakerError.js";
import type { ZobozBam } from "@shared/domain/services/ZobozBam.js";
import * as process from "process";
import { PackageJsonVerificationError } from "../domain/errors/PackageJsonVerificationError.js";
import type { BuildOrchestrator } from "../domain/interfaces/BuildOrchestrator.js";
import type { BuildOrchestratorResult } from "../domain/valueObjects/BuildOrchestratorResult.js";
import { PackageJsonExpectation } from "../domain/valueObjects/PackageJsonExpectation.js";
import { logger } from "../supporting/logger.js";

export class BuildsOrchestrator {
	constructor(
		private readonly zobozBam: ZobozBam,
		private readonly orchestrators: BuildOrchestrator[],
	) {
		if (orchestrators.length === 0) {
			throw new Error("No orchestrators provided");
		}
	}

	async build(shouldUpdatePackageJson: boolean) {
		try {
			const results = await this.buildAllOrchestrators();
			const packageJsonExpectation = this.mergePackageJsonExpectations(results);

			if (shouldUpdatePackageJson) {
				await packageJsonExpectation.updatePackageJson();
			} else {
				await packageJsonExpectation.verifyPackageJson();
			}

			await this.zobozBam.verifyPackageJson({
				canUpdatePackageJson: shouldUpdatePackageJson,
			});

			await this.zobozBam.exit();
			logger.success("zoboz build complete");
		} catch (error) {
			if (error instanceof PackageJsonVerificationError) {
				logger.error(error.message);
			} else if (error instanceof DealBreakerError) {
				logger.error(error.message);
			} else {
				logger.error("Build failed", error);
			}

			process.exit(1);
		}
	}

	private mergePackageJsonExpectations(results: BuildOrchestratorResult[]) {
		return PackageJsonExpectation.mergeAll(
			results.map((result) => result.packageJsonExpectation),
		);
	}

	private async buildAllOrchestrators() {
		const results: BuildOrchestratorResult[] = [];
		for (const orchestrator of this.orchestrators) {
			results.push(await orchestrator.build());
		}
		return results;
	}
}
