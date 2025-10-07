export type Item = {
  id: string;
  name: string;
  description?: string;
  price: number;
  createdAt: Date;
  updatedAt: Date;
};

export const tryParseItem = (data: any): Item | null => {
  if (
    typeof data.id === 'string' &&
    typeof data.name === 'string' &&
    (typeof data.description === 'string' || data.description === undefined) &&
    typeof data.price === 'number' &&
    typeof data.createdAt === 'string' &&
    typeof data.updatedAt === 'string'
  ) {
    return {
      id: data.id,
      name: data.name,
      description: data.description,
      price: data.price,
      createdAt: new Date(data.createdAt),
      updatedAt: new Date(data.updatedAt),
    };
  }
  return null;
};
