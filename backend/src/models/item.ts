import { itemsTable } from '../db/schema';

export type Item = typeof itemsTable.$inferSelect;
export type NewItem = typeof itemsTable.$inferInsert;
export type ItemUpdate = Partial<typeof itemsTable.$inferInsert>;
