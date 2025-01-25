// auto-generated by ./scripts/hydrate.sh
export function map<T, U>(callback: (item: T) => U, array: T[]): U[] {
	const result: U[] = [];
	for (const item of array) {
		result.push(callback(item));
	}
	return result;
}
