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
      <h1 class="max-6-xs my-16 text-6xl font-thin text-sky-700 uppercase">Hello world!</h1>
      <For each={items()}>{(item: ItemModel) => <Item item={item} />}</For>
    </main>
  );
}
