import { join } from "@std/path/join";


/**
 * Convert a hex string (e.g. "1BAE") to a decimal HTML entity, e.g. "&#7094;"
 */
function hexToDecimalEntity(hex) {
  const dec = parseInt(hex, 16);
  return `&#${dec};`;
}

function transformContent(content) {
  const re = /\\u\{([0-9A-Fa-f]+)\}|\\u([0-9A-Fa-f]+)/g;

  const replaced = content.replace(re, (_, g1, g2) => {
    const hex = g1 ?? g2; 
    const up = hex.toUpperCase();
    return hexToDecimalEntity(up);
  });

  return replaced;
}
const cwd = Deno.cwd();
const guideDir = join(cwd, "guides");

async function walk(pth){
    for await (const each of Deno.readDir(pth)) {
        const eachPth = join(pth, each.name)
        if (each.isSymlink) {
            continue
        } else if (each.isDirectory) {
            await walk(eachPth)
        }
        const content = await Deno.readTextFile(eachPth)
        Deno.writeTextFile(eachPth, transformContent(content))
        console.log(each.name)
    }
}
await walk(guideDir)