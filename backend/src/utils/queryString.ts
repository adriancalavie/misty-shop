export function asInt<QueryStringInteger extends string>(
  value: QueryStringInteger,
  defaultValue: number = 0
): number {
  const parsed = parseInt(value, 10);
  return isNaN(parsed) ? defaultValue : parsed;
}
