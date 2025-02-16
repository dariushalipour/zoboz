import type { BuildConfig } from "@shared/domain/valueObjects/BuildConfig.js";
import { logger } from "@shared/supporting/logger.js";
import { filesRepository, zobozBam } from "container.js";
import * as process from "process";
import { CjsBuildOrchestrator } from "../../cjs/app/CjsBuildOrchestrator.js";
import { DtsBuildOrchestrator } from "../../dts/app/DtsBuildOrchestrator.js";
import { EsmBuildOrchestrator } from "../../esm/app/EsmBuildOrchestrator.js";
import { BuildsOrchestrator } from "../app/BuildsOrchestrator.js";
import { DistEmptier } from "../domain/services/DistEmptier.js";

export async function build(
	config: BuildConfig,
	shouldUpdatePackageJson: boolean,
): Promise<void> {
	const distEmptier = new DistEmptier(filesRepository);

	const orchestrators = [
		config.dts &&
			new DtsBuildOrchestrator(
				zobozBam,
				filesRepository,
				distEmptier,
				config.exports,
				config.dts,
				config.srcDir,
				config.distDir,
			),
		config.esm &&
			new EsmBuildOrchestrator(
				zobozBam,
				filesRepository,
				distEmptier,
				config.exports,
				config.esm,
				config.srcDir,
				config.distDir,
			),
		config.cjs &&
			new CjsBuildOrchestrator(
				zobozBam,
				filesRepository,
				distEmptier,
				config.exports,
				config.cjs,
				config.srcDir,
				config.distDir,
			),
	].filter(<A>(x: A | null): x is A => x !== null);

	if (orchestrators.length === 0) {
		logger.error("No cjs/esm/dts build config provided.");
		logger.hint(
			"BuildConfig in zoboz.config.ts has esm, cjs, and dts set to null.",
		);
		logger.hint(
			"Set at least one of them to a non-null value to have an effective build.",
		);
		process.exit(1);
	}

	const buildsOrchestrator = new BuildsOrchestrator(zobozBam, orchestrators);

	try {
		await buildsOrchestrator.build(shouldUpdatePackageJson);
	} catch (error) {
		const errorMessage = error instanceof Error ? error.message : String(error);
		logger.error("Build failed:", errorMessage);
	}
}
