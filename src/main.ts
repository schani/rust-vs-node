type Value = string;

interface Listener {
  setDirty(): void;
}

interface ValueProducer {
  getCurrent(): Value;
  addListener(l: Listener): void;
}

class Row implements ValueProducer {
  private readonly _listeners: Listener[] = [];

  constructor(private _data: Value) {}

  public set(v: Value): void {
    this._data = v;

    for (const l of this._listeners) {
      l.setDirty();
    }
  }

  public getCurrent(): Value {
    return this._data;
  }

  public addListener(l: Listener): void {
    this._listeners.push(l);
  }
}

class ToLowercase implements Listener, ValueProducer {
  private _current: Value | undefined;

  constructor(private readonly _input: ValueProducer) {}

  public setDirty(): void {
    this._current = undefined;
  }

  public getCurrent(): Value {
    if (this._current === undefined) {
      this._current = this._input.getCurrent().toLowerCase();
    }
    return this._current;
  }

  public addListener(_l: Listener): void {
    return;
  }
}

interface Query {
  readonly equalTo: Value;
}

class NoIndex {
  constructor(private readonly _rows: readonly ValueProducer[]) {}

  public runQuery(q: Query): readonly number[] {
    const result: number[] = [];
    for (let i = 0; i < this._rows.length; i++) {
      const vp = this._rows[i];
      if (vp.getCurrent() === q.equalTo) {
        result.push(i);
      }
    }
    return result;
  }
}

function makeToLowercase(input: ValueProducer): ValueProducer {
  const tl = new ToLowercase(input);
  input.addListener(tl);
  return tl;
}

function main(): void {
  for (let i = 0; i < 100000; i++) {
    const r1 = new Row("Foo");
    const r2 = new Row("BAr");
    const tl = makeToLowercase(r1);
    const idx = new NoIndex([tl, r2]);

    let result = idx.runQuery({ equalTo: "quux" });
    if (result.length !== 0) {
      throw new Error("Failed");
    }
    for (let j = 0; j < 100; j++) {
      r1.set("quuX");
      result = idx.runQuery({ equalTo: "quux" });
      if (result.length !== 1 || result[0] !== 0) {
        throw new Error("Failed");
      }
      r1.set("BOO");
      result = idx.runQuery({ equalTo: "boo" });
      if (result.length !== 1 || result[0] !== 0) {
        throw new Error("Failed");
      }
    }
  }
}

main();
