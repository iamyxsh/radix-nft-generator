<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import {  rdt } from "$lib/stores";
  import axios from 'axios'


  let name = ""
  let description = ""
  let image_uri = ""



  const submitMetadata = async () => {
    let manifest = await axios.post("http://127.0.0.1:3000/manifest", {
      name, description, image_uri
    }).then(res => res.data)
    // Need help here
    manifest = manifest + `
      CALL_METHOD
          Address("account_tdx_2_12yn74xkdhnj5tdqdr06wdke68f6vfs56c372fltg873j2avq759x3p")
          "try_deposit_batch_or_abort"
          Expression("ENTIRE_WORKTOP")
          Enum<0u8>()
      ;
    `
    console.log(manifest)
    const result = await $rdt!.walletApi.sendTransaction({
      transactionManifest: manifest,
    })

if (result.isErr()) {
  console.log(result)
}

await result.asyncMap((res) => {
  console.log(res.transactionIntentHash)
  return "" as any
})


  }
</script>

<section class="flex items-start flex-col mr-auto px-5 gap-10">
  <h1 class="text-start text-3xl">Welcome to the Developer Console</h1>

    <div class="border-[0.5px] rounded p-5 border-gray-300 min-w-[50vw] flex flex-col justify-start items-start gap-2 min-h-1/2">
      <h1 class="text-xl">NFT Generator</h1>
      <p1 class="mb-5">Please enter the NFT Details below</p1>
      <div class="flex justify-start gap-12 w-full mb-2">
        <input bind:value={name} class="w-1/3 p-1" placeholder="Token Name" />
        <input bind:value={image_uri} class="w-1/3 p-1" placeholder="Image URL"/>
      </div>
      <input bind:value={description} class="w-3/4 p-1" placeholder="Description"/>
      <Button on:click={submitMetadata}>Submit</Button>
    </div>

</section>


