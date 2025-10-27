import path from "path";
import { promises as fs } from "fs";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const distDir = path.resolve(__dirname, "../dist");

async function fixSourceMaps() {
  const files = await fs.readdir(distDir);

  for (const file of files) {
    if (file.endsWith(".map")) {
      const fullPath = path.join(distDir, file);
      const raw = await fs.readFile(fullPath, "utf-8");
      const map = JSON.parse(raw);

      if (Array.isArray(map.sources)) {
        map.sources = map.sources.map((sourcePath: string) => path.relative(distDir, sourcePath).replace(/\\/g, "/"));
        await fs.writeFile(fullPath, JSON.stringify(map), "utf-8");
        console.log(`✅ fixed: ${file}`);
      }
    }
  }
}

fixSourceMaps().catch((e) => {
  console.error("❌ Error:", e);
});
