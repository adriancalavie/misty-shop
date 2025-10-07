import { Item as ItemModel } from '~/models/item';

export type ItemProps = {
  item: ItemModel;
};

export const Item = ({ item }: ItemProps) => {
  return (
    <div class="m-2 rounded border p-4 shadow transition hover:shadow-lg text-left w-full max-w-md mx-auto">
      <div>{item.name}</div>
      <div>{item.description}</div>
      <div>${item.price.toFixed(2)}</div>
    </div>
  );
};
