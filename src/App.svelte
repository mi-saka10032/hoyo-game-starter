<script lang="ts">
  import Header from "@/layouts/Header.svelte";
  import Game from "@/layouts/Game.svelte";
  import HonKai3BG from "./assets/bh3.jpg";
  import YuanShenBG from "./assets/ys.jpg";
  import StarRailBG from "./assets/star.jfif";
  import { getSyncStatus, setSyncStatus } from "./lib";
  import { onMount } from "svelte";

  const games: GameItem[] = [
    {
      key: "bh3",
      bg: HonKai3BG,
      gameEnName: "Honkai Impact 3 or HYG Launcher",
      gameCnName: "崩坏3",
      processName: "BH3.exe",
    },
    {
      key: "ys",
      bg: YuanShenBG,
      gameEnName: "Genshin Impact or HYG Launcher",
      gameCnName: "原神",
      processName: "YuanShen.exe",
    },
    {
      key: "star",
      bg: StarRailBG,
      gameEnName: "Star Rail or HYG Launcher",
      gameCnName: "崩坏：星穹铁道",
      processName: "StarRail.exe",
    },
  ];

  let syncLauncher: boolean = false;

  function changeSyncStatus(event: CustomEvent<boolean>) {
    const status = event.detail;
    syncLauncher = status;
    setSyncStatus(status);
  }

  onMount(() => {
    syncLauncher = getSyncStatus();
  });
</script>

<root class="flex flex-col h-full">
  <Header value={syncLauncher} on:change={changeSyncStatus} />
  <main class="flex-1 grid grid-cols-3 text-white">
    {#each games as item, i (item.key)}
      <Game {...item} {syncLauncher} />
    {/each}
  </main>
</root>
