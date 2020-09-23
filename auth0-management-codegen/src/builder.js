const Builder = require("./codegen");
const {toSnake, toPascal} = require("./utils");

const OPERATION_NAME_PATCHES = [
  [/^get(.*)/, "get$1"]
]

const TYPES = {
  "string": "String",
  "integer": "i32",
  "boolean": "bool",
}

class Context {
  constructor() {
    this.scopes = {};
  }

  scope(tags) {
    const name = toSnake(tags[0]);
    return this.scopes[name] = this.scopes[name] || new Builder(name);
  }
}

function build(api) {
  const ctx = new Context();

  Object
    .keys(api.paths)
    .forEach(x => buildOperations(ctx, api.paths[x]))

  return ctx;
}

function buildOperations(ctx, operations) {
  const get = operations.get;
  if (get) {
    if (get.parameters && get.parameters.length > 1 && get.parameters[0].in === "query") {
      buildFindOperation(ctx, get);
    } else {
      buildGetOperation(ctx, get);
    }
  }
}

function buildGetOperation(ctx, operation) {
  const scope = ctx.scope(operation.tags);
  const name = getOpName(operation.operationId);
  const fn = scope
    .fn(name)
    .docs(getOpDocs(operation));

  if (operation.parameters) {
    for (const param of operation.parameters) {
      fn.param(param.name + ": i32");
    }
  }
}

function buildFindOperation(ctx, operation) {
  const scope = ctx.scope(operation.tags);
  const optsName = `Find${operation.operationId.substring(3)}`;
  const opts = scope
    .struct(optsName)
    .vis("pub");

  for (const param of operation.parameters) {
    if (param.in !== "query")
      throw new Error(`Invalid param type in operation ${operation.operationId}`);

    opts
      .field(param.name)
      .type(param.schema.type)
      .doc(param.description)
  }

  getOpFn(ctx, operation)
    .param(`opts: ${optsName}`);
}

function getOpFn(ctx, operation) {
  const scope = ctx.scope(operation.tags);
  const name = getOpName(operation.operationId);

  return scope
    .fn(name)
    .vis("pub")
    .docs(getOpDocs(operation));
}

function getOpName(name) {
  return toSnake(
    OPERATION_NAME_PATCHES
      .reduce(
        (result, match) => result.replace(match[0], match[1]),
        name
      )
  )
}

function getOpDocs(operation) {
  const docs = operation.description
    ? operation.description.split("\n")
    : [];

  if (operation.parameters) {
    docs.push("# Arguments");

    for (const param of operation.parameters) {
      docs.push(` * \`${param.name}\` - ${param.description}`);
    }
  }

  return docs;
}

module.exports = build;
