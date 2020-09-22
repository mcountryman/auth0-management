const {convert} = require("api-spec-converter");
const SwaggerParser = require("swagger-parser");
const Builder = require("./builder");

async function main() {
  const document = await convert({
    from: "swagger_1",
    to: "swagger_2",
    source: "https://login.auth0.com/api/v2/api-docs"
  });

  document.fillMissing();
  document.spec.paths["/api/v2/branding/templates/universal-login"].put.parameters[0].schema = undefined;
  document.spec.paths["/api/v2/branding/templates/universal-login"].put.parameters[0].type = "string";

  const api = await SwaggerParser.dereference(document.spec);
  const module = new Builder("test")
    .struct("Test")
      .field("test")
        .vis("pub")
        .type("String")
        .build()
    .build()

  console.log(module.toString());
  // console.log(api.paths[Object.keys(api.paths)[0]].get.responses['200'].schema);
}

main().then(() => console.log("Done.")).catch(console.error);