import { Hono } from 'hono';
import { createItem, deleteItem, getItemById, getItems, updateItem } from '../stores/items';
import { parseNumber } from '../utils/queryString';

const app = new Hono();

app.get('/', c => {
  const { limit, offset } = c.req.query();
  return c.json(getItems({ limit: parseNumber(limit), offset: parseNumber(offset) }));
});

app.post('/', async c => {
  const body = await c.req.json();
  return c.json(createItem(body));
});

app.get('/:id', c => {
  const { id } = c.req.param();
  return c.json(getItemById(id));
});

app.put('/:id', async c => {
  const { id } = c.req.param();
  const body = await c.req.json();
  return c.json(updateItem(id, body));
});

app.delete('/:id', c => {
  const { id } = c.req.param();
  return c.json(deleteItem(id));
});

export default app;
