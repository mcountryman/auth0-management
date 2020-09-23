class BaseBuilder {
  constructor(name, parent, canImport) {
    this._vis = "";
    this._docs = [];
    this._name = name;
    this._attrs = [];
    this._imports = {};
    this.parent = parent;
    this._canImport = canImport;
  }

  vis(value) {
    this._vis = value;
    return this;
  }

  attr(value) {
    this._attrs.push(value);
    return this;
  }

  doc(value) {
    this._docs.push(value);
    return this;
  }

  docs(value) {
    this._docs = value;
    return this;
  }

  use(path, name) {
    if (this._canImport) {
      if (this._imports[path]) {
        this._imports[path].push(name);
      } else {
        this._imports[path] = [name];
      }
    } else {
      if (this.parent) {
        this.parent.use(path, name);
      } else {
        throw new Error(`Couldn't import '${path}::${name}'`);
      }
    }

    return this;
  }

  build() {
    return this.parent;
  }

  toString() {
    return this._toStringFromLines(this._getLines(), 0);
  }

  _toStringFromLines(lines, depth) {
    let result = [];

    for (const line of lines) {
      if (Array.isArray(line)) {
        result.push(this._toStringFromLines(line, depth));
      } else if (typeof line === "object" && "_toStringFromLines" in line) {
        result.push(line._toStringFromLines(line._getLines(), depth + 1));
      } else if (typeof line === "string" && line.length > 0) {
        let fuck = "";
        for (let i = 0; i < depth; i++)
          fuck += "  ";

        result.push(fuck + line);
      }
    }

    return result.join("\n");
  }

  _getDepth() {
    let depth = 0;
    let parent = this.parent;

    for (;;) {
      if (parent) {
        depth++;
        parent = parent.parent;
      } else {
        return depth;
      }
    }
  }

  _getLines() {
    return [
      ...this._docs.map(x => "/// " + x),
      ...this._attrs,
    ];
  }

  _getLine(...args) {
    return args
      .filter(x => x && x.length > 0)
      .join(" ");
  }

  _getImports() {
    return Object
      .keys(this._imports)
      .map(path => `use ${path}::{${this._imports[path].join(", ")}};`)
      .concat([" "])
  }
}

class ScopeBuilder extends BaseBuilder {
  constructor(name) {
    super(name, undefined, true);

    this._fns = [];
    this._impls = [];
    this._traits = [];
    this._modules = [];
    this._structs = [];
  }

  struct(name) {
    const struct = new StructBuilder(name, this);
    this._structs.push(struct);
    return struct;
  }

  fn(name) {
    const fn = new FnBuilder(name, this);
    this._fns.push(fn);
    return fn;
  }

  mod(name) {
    const module = new ModuleBuilder(name, this);
    this._modules.push(module);
    return module;
  }

  toString() {
    return this._toStringFromLines(this._getLines(), -1);
  }

  _getLines() {
    return [
      ...super._getLines(),
      ...super._getImports(),
    ]
      .concat(this._fns)
      .concat(this._structs)
      .concat(this._modules);
  }
}

class ModuleBuilder extends ScopeBuilder {
  constructor(name, parent) {
    super(name, parent, true);
  }

  _getLines() {
    return [
      ...super._getLines(),
      ...super._getImports(),
      this._getLine(this._vis, "mod", this._name, "{"),
      this._structs,
      "}",
      " "
    ]
  }
}

class StructBuilder extends BaseBuilder {
  constructor(name, parent) {
    super(name, parent);
    this.fields = [];
  }

  field(name) {
    const field = new FieldBuilder(name, this);
    this.fields.push(field);
    return field;
  }

  _getLines() {
    return [
      ...super._getLines(),
      this._getLine(this._vis, "struct", this._name, "{"),
      this.fields,
      "}",
      " "
    ];
  }
}

class FnBuilder extends BaseBuilder {
  constructor(name, parent) {
    super(name);

    this._name = name;
    this._type = "";
    this._lines = [];
    this._params = [];

    this.parent = parent;
  }

  line(value) {
    this._lines.push(value);
    return this;
  }

  param(value) {
    this._params.push(value);
    return this;
  }

  _getLines() {
    const ret = this._type.length > 0 ? " -> " + this._type : "";
    const params = "(" + this._params.join(", ") + ")";
    return [
      ...super._getLines(),
      this._getLine(this._vis, "fn", this._name + params + ret),
      this._lines,
      "}",
      " "
    ];
  }
}

class FieldBuilder extends BaseBuilder {
  constructor(name, parent) {
    super(name);

    this._type = "i32";
    this.parent = parent;
  }

  type(value) {
    this._type = value;
    return this;
  }

  _getLines() {
    return [
      ...super._getLines(),
      this._getLine(this._vis, this._name + ":", this._type),
    ]
  }
}


module.exports = ScopeBuilder;
