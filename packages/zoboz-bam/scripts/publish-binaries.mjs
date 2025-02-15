#!/usr/bin/env node

import { spawnSync } from "child_process";
import fs from "fs";
import path from "path";
import { env, platform } from "process";

const packages = [
	{
		name: "@zoboz/bam-bin-darwin-arm64",
		target: "aarch64-apple-darwin",
		binary: "zoboz-bam-darwin-arm64",
		cpu: "arm64",
		os: "darwin",
	},
	{
		name: "@zoboz/bam-bin-win32-arm64",
		target: "aarch64-pc-windows-msvc",
		binary: "zoboz-bam-win32-arm64.exe",
		cpu: "arm64",
		os: "win32",
	},
	{
		name: "@zoboz/bam-bin-linux-arm64",
		target: "aarch64-unknown-linux-musl",
		binary: "zoboz-bam-linux-arm64",
		cpu: "arm64",
		os: "linux",
	},
	{
		name: "@zoboz/bam-bin-darwin-x64",
		target: "x86_64-apple-darwin",
		binary: "zoboz-bam-darwin-x64",
		cpu: "x64",
		os: "darwin",
	},
	{
		name: "@zoboz/bam-bin-win32-x64",
		target: "x86_64-pc-windows-msvc",
		binary: "zoboz-bam-win32-x64.exe",
		cpu: "x64",
		os: "win32",
	},
	{
		name: "@zoboz/bam-bin-linux-x64",
		target: "x86_64-unknown-linux-musl",
		binary: "zoboz-bam-linux-x64",
		cpu: "x64",
		os: "linux",
	},
];

const readmeContent =
	"CAUTION: This is a binary package. It is not intended to be used directly. It is a dependency of the @zoboz/bam package. Please refer to the main package for more information.";

const binariesDir = path.join(import.meta.dirname, "..", "binaries");
const binPackagesDir = path.join(import.meta.dirname, "..", "bin-packages");

const mainPackageVersion = JSON.parse(
	fs.readFileSync(path.join(import.meta.dirname, "..", "package.json")),
).version;

validateBinariesDirectory();

fs.rmSync(binPackagesDir, { recursive: true });

for (const pkg of packages) {
	const packageDir = path.join(binPackagesDir, pkg.name);

	fs.mkdirSync(packageDir, { recursive: true });

	const packageJson = generatePackageJson(pkg);

	fs.writeFileSync(
		path.join(packageDir, "package.json"),
		JSON.stringify(packageJson, null, 2),
	);

	fs.writeFileSync(path.join(packageDir, "README.md"), readmeContent);

	fs.copyFileSync(
		path.join(binariesDir, pkg.binary),
		path.join(packageDir, pkg.binary),
	);

	executeNpmPublish(packageDir);
}

function executeNpmPublish(packageDir) {
	const npm = platform === "win32" ? "npm.cmd" : "npm";
	spawnSync(
		npm,
		[
			"publish",
			...("PUBLISH_REGISTRY" in env
				? ["--registry", env.PUBLISH_REGISTRY]
				: []),
		],
		{
			cwd: packageDir,
			stdio: "inherit",
		},
	);
}

function generatePackageJson(pkg) {
	return {
		name: pkg.name,
		version: mainPackageVersion,
		private: false,
		publishConfig: {
			access: "public",
		},
		repository: {
			type: "git",
			url: "git+https://github.com/dariushalipour/zoboz.git",
		},
		license: "MIT",
		preferUnplugged: true,
		os: [pkg.os],
		cpu: [pkg.cpu],
		files: [pkg.binary],
		main: pkg.binary,
		scripts: {},
		keywords: [],
	};
}

function validateBinariesDirectory() {
	const binaries = fs.readdirSync(binariesDir);
	const missingBinaries = packages.filter(
		(pkg) => !binaries.includes(pkg.binary),
	);

	if (missingBinaries.length > 0) {
		const missingOnes = missingBinaries.map((pkg) => pkg.binary).join(", ");
		throw new Error(
			`The following binaries are missing in the binaries directory: ${missingOnes}`,
		);
	}
}
