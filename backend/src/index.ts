import { drizzle } from 'drizzle-orm/bun-sqlite';
import { Hono } from 'hono';
import { cors } from 'hono/cors';
import items from './handlers/items';

const db = drizzle(process.env.DB_FILE_NAME!);

const app = new Hono();

app.use('/*', cors());
app.route('/items', items);

export default app;
