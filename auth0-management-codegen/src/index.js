const {convert} = require("api-spec-converter");
const SwaggerParser = require("swagger-parser");
const build = require("./builder");
const Builder = require("./codegen");
const fs = require("fs").promises;

async function main() {
  const document = await convert({
    from: "swagger_1",
    to: "swagger_2",
    source: "https://login.auth0.com/api/v2/api-docs"
  });

  document.fillMissing();
  document.spec.paths["/api/v2/branding/templates/universal-login"].put.parameters[0].schema = undefined;
  document.spec.paths["/api/v2/branding/templates/universal-login"].put.parameters[0].type = "string";

  const document2 = await convert({
    from: "swagger_2",
    to: "openapi_3",
    source: document.spec
  });

  const api = await SwaggerParser.dereference(document2.spec);
  const res = build(api);

  // console.log(Object
  //   .keys(api.paths)
  //   .map(x => api.paths[x])
  //   .filter(x => x.get && x.get.parameters && x.get.parameters.length > 0)
  //   .map(x => x.get.parameters)
  //   [0]
  // )

  console.log(res.scopes[Object.keys(res.scopes)[1]].toString())
  // console.log(Object.keys(res.scopes).map(x => res.scopes[x].toString()).join("\n\n"));
}

main().then(() => console.log("Done.")).catch(console.error);