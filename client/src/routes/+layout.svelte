<script lang="ts">
  import  '../app.css'
  import { GatewayApiClient } from "@radixdlt/babylon-gateway-api-sdk";
  import {
    DataRequestBuilder,
    RadixDappToolkit,
    RadixNetwork,
  } from "@radixdlt/radix-dapp-toolkit";
  import { onMount } from "svelte";
  import { gatewayApi, rdt, walletData } from "$lib/stores";
  import { dAppId } from "$lib/constants";
  import Nav from "./Nav.svelte";

  onMount(() => {

    const applicationVersion = "1.0.0";
    const applicationName = "Hello Token dApp";
    const networkId = RadixNetwork.Stokenet;


    $gatewayApi = GatewayApiClient.initialize({
      networkId,
      applicationName,
      applicationVersion,
    });
    console.log("gatewayApi: ", $gatewayApi);


    $rdt = RadixDappToolkit({
      dAppDefinitionAddress: dAppId,
      networkId,
      applicationName,
      applicationVersion,
    });
    console.log("dApp Toolkit: ", $rdt);


    $rdt?.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1));


    $rdt?.walletApi.walletData$.subscribe((data) => {
      $walletData = data;
    });
  });
</script>

<Nav />

<main>
  <slot />
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5rem;
    padding: 5rem 0;
  }
</style>
