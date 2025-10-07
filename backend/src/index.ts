import { Hono } from 'hono';
import { cors } from 'hono/cors';
import items from './handlers/items';

const app = new Hono();

app.route('/items', items);
app.use('/*', cors());

export default app;
