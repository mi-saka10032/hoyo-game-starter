<script lang="ts">
	import Header from './lib/Header.svelte';
  import Game from "./lib/Game.svelte";
  import HonKai3BG from "./assets/bh3.jpg";
  import YuanShenBG from "./assets/ys.jpg";
  import StarRailBG from "./assets/star.jfif";

  const games: GameItem[] = [
    {
      key: "bh3",
      bg: HonKai3BG,
      gameEnName: "Honkai Impact 3",
      gameCnName: "崩坏3",
      processName: "BH3.exe",
    },
    {
      key: "ys",
      bg: YuanShenBG,
      gameEnName: "Genshin Impact",
      gameCnName: "原神",
      processName: "YuanShen.exe",
    },
    {
      key: "star",
      bg: StarRailBG,
      gameEnName: "Star Rail",
      gameCnName: "崩坏：星穹铁道",
      processName: "StarRail.exe",
    },
  ];

  let timer: number = 0;
</script>

<root class="flex flex-col h-full">
  <Header />
  <main class="flex-1 grid grid-cols-3 text-white">
    {#each games as item, i (item.key)}
      <Game {...item} />
    {/each}
  </main>
</root>
