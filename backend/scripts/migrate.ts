import { existsSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { Database } from 'bun:sqlite';
import { drizzle } from 'drizzle-orm/bun-sqlite';
import { migrate } from 'drizzle-orm/bun-sqlite/migrator';

const dbFileName = process.env.DB_FILE_NAME!;
const cwd = process.cwd();
const resolvedDbFile = isAbsolute(dbFileName) ? dbFileName : join(cwd, dbFileName);
const migrationsFolder = join(cwd, 'drizzle');

const sqlite = new Database(resolvedDbFile, { create: true });
const db = drizzle(sqlite);

async function runMigrations() {
  console.log(`Running migrations on ${resolvedDbFile}`);

  if (!existsSync(migrationsFolder)) {
    console.warn(`Migrations folder not found at ${migrationsFolder}. Nothing to apply.`);
    return;
  }

  migrate(db, { migrationsFolder });

  console.log('Migrations completed successfully.');
}

// We need this workaround instead of `bunx drizzle-kit migrate`
// See https://github.com/drizzle-team/drizzle-orm/issues/3423
runMigrations()
  .then(() => {
    sqlite.close();
  })
  .catch(error => {
    console.error('Migration failed:', error);
    sqlite.close();
    process.exit(1);
  });
