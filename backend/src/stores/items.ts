import { Item } from '../models/item';

// Mock
export type GetOptions = {
  limit?: number;
  offset?: number;
};

const items: Item[] = [];

export function getItems(opts: GetOptions): Item[] {
  const offset = opts.offset || 0;
  const limit = opts.limit ? offset + opts.limit : undefined;
  return items.slice(offset, limit);
}
export function getItemById(id: string): Item | null {
  return items.find(item => item.id === id) || null;
}
export function createItem(data: any): Item {
  const newItem: Item = {
    id: crypto.randomUUID(),
    name: data.name,
    description: data.description,
    price: data.price,
    createdAt: new Date(),
    updatedAt: new Date(),
  };
  items.push(newItem);
  return newItem;
}
export function updateItem(id: string, data: any): Item | null {
  const item = items.find(item => item.id === id);
  if (!item) return null;
  Object.assign(item, data, { updatedAt: new Date() });
  return item;
}
export function deleteItem(id: string): boolean {
  const index = items.findIndex(item => item.id === id);
  if (index === -1) return false;
  items.splice(index, 1);
  return true;
}
