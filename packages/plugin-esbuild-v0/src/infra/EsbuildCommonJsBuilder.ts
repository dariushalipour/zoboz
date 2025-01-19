import { type Builder, type OutDir, type SrcDir, logger } from "@zoboz/core";
import * as esbuild from "esbuild";
import type { EsbuildOptions } from "../domain/interfaces/EsbuildOptions.js";

export class EsbuildCommonJsBuilder implements Builder {
	constructor(private readonly buildOptions?: EsbuildOptions) {}

	async build(srcDir: SrcDir, outDir: OutDir): Promise<void> {
		logger.pending(`Building CommonJS by esbuild to ${outDir.uri}`);

		await esbuild.build({
			entryPoints: ["./src/**/*.ts"],
			outdir: outDir.absoluteUri,
			format: "cjs",
			platform: "node",
			...(this.buildOptions ?? {}),
		});
	}
}
