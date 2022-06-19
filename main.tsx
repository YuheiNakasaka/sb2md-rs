import init, { ToMd } from "./sb2md_converter/pkg/converter";

init().then(async () => {
  const text = "- this is a [* test]. In details, [https://example.com/ link] should be shown. [https://scrapbox.io/files/test.png]";
  const tomd = await ToMd.new(text);
  const res = await tomd.convert();
  console.log(`result: ${res}`);
});