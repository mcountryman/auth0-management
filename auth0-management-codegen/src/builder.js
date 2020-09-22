
class BaseBuilder {
  constructor(name, parent) {
    this._vis = "";
    this._docs = "";
    this._name = name;
    this.parent = parent;
  }

  vis(value) {
    this._vis = value;
    return this;
  }

  docs(value) {
    this._docs = value;
    return this;
  }

  import(path, name) {
    if (this.parent) {
      this.parent.import(path, name);
    } else {
      throw new Error(`Couldn't import '${path}::${name}'`);
    }
  }

  build() {
    return this.parent;
  }

  toString() {
    return "///" + this._docs.split("\n").join("///\n");
  }

  _toStringFromLines(lines) {
    return lines
      .filter(line => line && line.length > 0)
      .join("\n");
  }
}

class ModuleBuilder extends BaseBuilder {
  constructor(name) {
    super(name);

    this._structs = [];
    this._imports = {};
  }

  import(path, name) {
    if (this._imports[path]) {
      this._imports[path].push(name);
    } else {
      this._imports[path] = [name];
    }
  }

  struct(name) {
    const struct = new StructBuilder(name, this);
    this._structs.push(struct);
    return struct;
  }

  toString() {
    return this._toStringFromLines([
      super.toString(),
      `${this._vis} mod ${this._name} {`,
      // Object
      //   .keys(this._imports)
      //   .map(path => `  use ${path}::\{${this._imports[path].join(", ")}\};`)
      this._structs.map(struct => struct.toString()),
      `}`
    ].flat());
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

  toString() {
    return super._toStringFromLines([
      super.toString(),
      `${this._vis} struct ${this._name} {`,
      this.fields.map(field => field.toString()),
      `}`,
    ].flat())
  }
}

class FnBuilder extends BaseBuilder {
  constructor(name, parent) {
    super(name);

    this._name = name;
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

  toString() {
    return super._toStringFromLines([
      super.toString(),
      this._attr,
      `  ${this._vis} fn ${this._name}(${this._params.join(", ")}) -> ${this._type} {`,
      this._lines,
      `}`,
    ].flat())
  }
}

class FieldBuilder extends BaseBuilder {
  constructor(name, parent) {
    super(name);

    this._attr = "";
    this._type = "i32";
    this.parent = parent;
  }

  attr(value) {
    this._attr = value;
    return this;
  }

  type(value) {
    this._type = value;
    return this;
  }

  toString() {
    return super._toStringFromLines([
      super.toString(),
      this._attr,
      `  ${this._vis} ${this._name}: ${this._type},`
    ])
  }
}


module.exports = ModuleBuilder;
