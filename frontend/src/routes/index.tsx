import { createResource, For } from 'solid-js';
import { getBaseUrl } from '~/api/utils';
import { Item } from '~/components/Item';
import { Item as ItemModel, tryParseItem } from '~/models/item';

const fetchItems = async (): Promise<ItemModel[]> => {
  const response = await fetch(`${getBaseUrl()}/items`);
  const data = await response.json();
  
  return data.map(tryParseItem).filter((item: ItemModel | null) => item !== null);
};

export default function Home() {
  const [items] = createResource(fetchItems);

  return (
    <main class="mx-auto p-4 text-center text-gray-700">
      <For each={items()}>{item => <Item item={item} />}</For>
    </main>
  );
}
