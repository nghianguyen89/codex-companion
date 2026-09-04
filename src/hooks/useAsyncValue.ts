import { useCallback, useEffect, useState } from "react";

interface AsyncState<T> {
  value: T | null;
  loading: boolean;
  error: string | null;
}

export function useAsyncValue<T>(load: () => Promise<T>): AsyncState<T> & { refresh: () => Promise<void>; setValue: (value: T) => void } {
  const [state, setState] = useState<AsyncState<T>>({ value: null, loading: true, error: null });

  const refresh = useCallback(async () => {
    setState((current) => ({ ...current, loading: true, error: null }));
    try {
      setState({ value: await load(), loading: false, error: null });
    } catch (error: unknown) {
      setState({
        value: null,
        loading: false,
        error: error instanceof Error ? error.message : "Unable to load data."
      });
    }
  }, [load]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const setValue = useCallback((value: T) => setState({ value, loading: false, error: null }), []);
  return { ...state, refresh, setValue };
}
