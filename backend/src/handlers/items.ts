import { Hono } from 'hono';
import { createItem, deleteItem, getItemById, getItems, updateItem } from '../stores/items';
import { cors } from 'hono/cors';

const app = new Hono();

app.get('/', async c => {
  const { limit, offset } = c.req.query();
  const result = await getItems({ limit, offset });
  return c.json(result);
});

app.post('/', async c => {
  const body = await c.req.json();
  const result = await createItem(body);
  return c.json(result);
});

app.get('/:id', async c => {
  const { id } = c.req.param();
  const result = await getItemById(id);
  return c.json(result);
});

app.put('/:id', async c => {
  const { id } = c.req.param();
  const body = await c.req.json();
  const result = await updateItem(id, body);
  return c.json(result);
});

app.delete('/:id', async c => {
  const { id } = c.req.param();
  const result = await deleteItem(id);
  return c.json(result);
});

app.use('/*', cors());

export default app;
