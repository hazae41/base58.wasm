import { assert, test } from "@hazae41/phobos"
import { Memory, base58_decode, base58_encode, initBundled } from "mods/index.js"

test("base58", async () => {
  await initBundled()

  using memory = new Memory(crypto.getRandomValues(new Uint8Array(256)))

  const encoded = base58_encode(memory)
  using decoded = base58_decode(encoded)

  assert(Buffer.from(decoded.bytes).equals(Buffer.from(memory.bytes)))
})