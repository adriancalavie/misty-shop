export function parseNumber<QueryString extends string>(value: QueryString): number | undefined {
  const parsed = parseInt(value, 10);
  return isNaN(parsed) ? undefined : parsed;
}
