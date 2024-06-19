<script lang="ts" name="Game">
  import { type UnlistenFn, listen, TauriEvent } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { checkResource } from "@/api";
  import { HoyoClass, HoyoInterface, getStorage, setStorage } from "@/lib";
  import VersionTag from "@/components/VersionTag.svelte";
  import Directory from "@/components/Directory.svelte";
  import StarterButton from "@/components/StarterButton.svelte";
  import OfficialButton from "@/components/OfficialButton.svelte";

  export let key: keyof typeof GameKey;
  export let bg: string;
  export let gameEnName: string;
  export let gameCnName: string;
  export let processName: string;

  let hoyoClass: HoyoClass = new HoyoClass(
    {
      root: "",
      launcher: "",
      game: "",
      exe: "",
    },
    processName
  );

  let timer: number = 0;

  let launcherValidation = false;

  let exeValidation = false;

  let processStatus: boolean = false;

  let version: string = "";

  let remoteVersion: string = "";

  let hasPreDownload: boolean = false;

  /** 游戏进程监听 */
  async function checkGameProcess() {
    const flag = await hoyoClass.checkGameStatus();
    if (flag) {
      // 进程开启
      HoyoClass.changeWindowStatus(false);
    } else {
      closeWatch();
      // 进程关闭
      HoyoClass.changeWindowStatus(true);
    }
    processStatus = flag;
  }

  function openWatch(delay: number = 1000) {
    timer = window.setInterval(checkGameProcess, delay);
  }

  function closeWatch() {
    window.clearInterval(timer);
    timer = 0;
  }

  /** 检查本地游戏版本 */
  async function checkLocalVersion() {
    const configText = await hoyoClass.readLocalVersion();
    const reg = /game_version=(.*)/;
    version = reg.exec(configText)?.[1] ?? "";
  }

  /** 检查当前在线版本 */
  async function checkRemoteVersion() {
    const result = await checkResource(key as GameKey);
    remoteVersion = result.remoteVersion ?? "";
    hasPreDownload = result.hasPreDownload;
  }

  /** 检查路径正确性 */
  async function checkPath() {
    [launcherValidation, exeValidation] = await Promise.all([
      hoyoClass.checkLauncherPathValid(),
      hoyoClass.checkExePathValid(),
    ]);
    if (launcherValidation || exeValidation) {
      const gameInfo = hoyoClass.getHoyoInterface();
      setStorage(key, gameInfo);
      checkLocalVersion();
      checkRemoteVersion();
    }
  }

  function handleSpecifyGamePath(event: CustomEvent<HoyoInterface>) {
    const result = event.detail;
    hoyoClass = new HoyoClass(result, processName);
    checkPath();
  }

  function handleWatchProcess() {
    closeWatch();
    openWatch();
  }

  let unListen: UnlistenFn | null = null;
  onMount(async () => {
    hoyoClass = new HoyoClass(getStorage(key), processName);
    checkPath();
    unListen = await listen(TauriEvent.WINDOW_FOCUS, checkLocalVersion);
  });

  onDestroy(() => {
    closeWatch();
    unListen && unListen();
  });
</script>

<section class="relative flex flex-col opacity-80 hover:opacity-100">
  <img src={bg} alt={gameEnName} class="w-full h-full" draggable="false" />
  <VersionTag {version} {remoteVersion} {hasPreDownload} />
  <Directory
    {gameEnName}
    {gameCnName}
    {hoyoClass}
    {launcherValidation}
    {exeValidation}
    {processStatus}
    on:specify-game-path={handleSpecifyGamePath}
  />
  {#if processStatus}
    <h2 class="absolute-middle w-full text-center text-amber-500">
      游戏已启动
    </h2>
  {/if}
  {#if !processStatus}
    <div
      class="absolute bottom-0 flex flex-col items-center space-y-4 mb-4 w-full"
    >
      {#if launcherValidation}
        <OfficialButton
          cls={hasPreDownload ||
          (version.length > 0 &&
            remoteVersion.length > 0 &&
            version !== remoteVersion)
            ? "animate-pulse"
            : ""}
          {hoyoClass}
        />
      {/if}
      {#if exeValidation}
        <StarterButton
          disabled={version !== remoteVersion}
          {hoyoClass}
          on:watch-process={handleWatchProcess}
        />
      {/if}
    </div>
  {/if}
</section>
