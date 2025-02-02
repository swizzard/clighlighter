export interface M<K, V> {
  get: (k: K) => V | undefined;
}
export function newMapObject<K, V>(): Map<K, V> {
  return new Map();
}

export function newMapFunction<K, V>(): M<K, V> {
  return {
    get: (_k: K) => {
      return undefined;
    },
  };
}

export function setMapObject<K, V>(m: Map<K, V>, k: K, v: V): Map<K, V> {
  return m.set(k, v);
}

export function setMapFunction<K, V>(m: M<K, V>, k: K, v: V): M<K, V> {
  return {
    get: (key: K) => (key === k ? v : m.get(k)),
  };
}

export function delMapObject<K, V>(m: Map<K, V>, k: K): Map<K, V> {
  m.delete(k);
  return m;
}

export function delMapFunction<K, V>(m: M<K, V>, k: K): M<K, V> {
  return setMapFunction(m, k, undefined) as M<K, V>;
}
