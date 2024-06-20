<script lang="ts" name="Game">
  import { type UnlistenFn, listen, TauriEvent } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { checkResource } from "@/api";
  import { HoyoClass, getStorage, setStorage, SYNC_EVENT_NAME } from "@/lib";
  import VersionTag from "@/components/VersionTag.svelte";
  import Directory from "@/components/Directory.svelte";
  import StarterButton from "@/components/StarterButton.svelte";
  import OfficialButton from "@/components/OfficialButton.svelte";

  export let key: keyof typeof GameKey;
  export let bg: string;
  export let gameEnName: string;
  export let gameCnName: string;
  export let processName: string;
  export let syncLauncher: boolean;

  const syncEvent = "sync-launcher";

  let hoyoClass: HoyoClass = new HoyoClass(
    {
      launcherPath: "",
      launcherFile: "",
      gamePath: "",
      gameFile: "",
      scriptPath: "",
      scriptFile: "",
    },
    processName
  );

  let timer: number = 0;

  let launcherValidation = false;

  let gameValidation = false;

  let scriptValidation = false;

  let processStatus: boolean = false;

  let version: string = "";

  let remoteVersion: string = "";

  let hasPreDownload: boolean = false;

  /** 游戏进程监听 */
  async function checkGameProcess() {
    const flag = await hoyoClass.checkGameStatus();
    if (flag && !processStatus) {
      // 进程开启
      HoyoClass.changeWindowStatus(false);
    } else if (!flag && processStatus) {
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

  function syncLauncherPath(event: Event) {
    const launcherProp = (event as unknown as CustomEvent<FileProp>).detail;
    const param: HoyoInterface = {
      launcherPath: launcherProp.path,
      launcherFile: launcherProp.file,
      gamePath: hoyoClass.gameProp.path,
      gameFile: hoyoClass.gameProp.file,
      scriptPath: hoyoClass.scriptProp.path,
      scriptFile: hoyoClass.scriptProp.file,
    };
    hoyoClass = new HoyoClass(param, processName);
  }

  /** 检查路径正确性 */
  async function checkPath() {
    [launcherValidation, gameValidation, scriptValidation] = await Promise.all([
      hoyoClass.checkLauncherPathValid(),
      hoyoClass.checkGamePathValid(),
      hoyoClass.checkScriptPathValid(),
    ]);
    if (launcherValidation && gameValidation && scriptValidation) {
      const gameInfo = hoyoClass.getHoyoInterface();
      setStorage(key, gameInfo);
      checkLocalVersion();
      checkRemoteVersion();
    }
    if (syncLauncher) {
      const customEvent = new CustomEvent(syncEvent, {
        detail: {
          path: hoyoClass.launcherProp.path,
          file: hoyoClass.launcherProp.file,
        },
      });
      window.dispatchEvent(customEvent);
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
    window.addEventListener(SYNC_EVENT_NAME, syncLauncherPath);
    const persistData = getStorage(key);
    hoyoClass = new HoyoClass(getStorage(key), processName);
    checkPath();
    unListen = await listen(TauriEvent.WINDOW_FOCUS, checkLocalVersion);
  });

  onDestroy(() => {
    window.removeEventListener(SYNC_EVENT_NAME, syncLauncherPath);
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
    {processStatus}
    exeFileValidation={gameValidation && scriptValidation}
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
      {#if gameValidation}
        <StarterButton {hoyoClass} on:watch-process={handleWatchProcess} />
      {/if}
    </div>
  {/if}
</section>
