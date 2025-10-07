import { drizzle } from 'drizzle-orm/bun-sqlite';
import { itemsTable } from '../db/schema';
import { Item, ItemUpdate, NewItem } from '../models/item';
import { asInt } from '../utils/queryString';
import { eq } from 'drizzle-orm';

const db = drizzle(process.env.DB_FILE_NAME!);

export async function getItems(opts: { limit: string; offset: string }): Promise<Item[]> {
  const { limit, offset } = {
    limit: asInt(opts.limit, 10),
    offset: asInt(opts.offset, 0),
  };
  return db.select().from(itemsTable).limit(limit).offset(offset);
}
export async function getItemById(id: string): Promise<Item | null> {
  const result = await db.select().from(itemsTable).where(eq(itemsTable.id, id));
  return result[0] || null;
}

export async function createItem(data: any): Promise<Item> {
  const item: NewItem = {
    name: data.name,
    description: data.description || null,
    price: data.price,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };
  const result = await db.insert(itemsTable).values(item).returning();
  return result[0];
}
export async function updateItem(id: string, data: any): Promise<Item | null> {
  const existingItem = await getItemById(id);
  if (!existingItem) return null;

  const updatedItem: ItemUpdate = { ...data, updatedAt: new Date().toISOString() };
  const result = await db
    .update(itemsTable)
    .set(updatedItem)
    .where(eq(itemsTable.id, id))
    .returning();

  return result[0] || null;
}

export async function deleteItem(id: string): Promise<boolean> {
  const result = await db.delete(itemsTable).where(eq(itemsTable.id, id)).returning();
  return result.length > 0;
}
