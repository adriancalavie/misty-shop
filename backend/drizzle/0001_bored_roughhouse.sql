PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_items_table` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`description` text,
	`price` integer NOT NULL,
	`created_at` text NOT NULL,
	`updated_at` text NOT NULL
);
--> statement-breakpoint
INSERT INTO `__new_items_table`("id", "name", "description", "price", "created_at", "updated_at") SELECT "id", "name", "description", "price", "created_at", "updated_at" FROM `items_table`;--> statement-breakpoint
DROP TABLE `items_table`;--> statement-breakpoint
ALTER TABLE `__new_items_table` RENAME TO `items_table`;--> statement-breakpoint
PRAGMA foreign_keys=ON;