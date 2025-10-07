import { int, sqliteTable, text } from 'drizzle-orm/sqlite-core';

export const itemsTable = sqliteTable('items_table', {
  id: int('id').primaryKey({ autoIncrement: true }),
  name: text('name').notNull(),
  description: text('description'),
  price: int('price').notNull(),
  createdAt: text('created_at').notNull(),
  updatedAt: text('updated_at').notNull(),
});