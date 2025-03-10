<script lang="ts" name="Directory">
  import { createEventDispatcher } from "svelte";
  import { KButton } from "@ikun-ui/button";
  import { KTooltip } from "@ikun-ui/tooltip";
  import { KMessage } from "@ikun-ui/message";
  import AppointButton from "@/components/AppointButton.svelte";
  import { HoyoClass } from "@/lib";

  export let gameEnName: string;

  export let gameCnName: string;

  export let hoyoClass: HoyoClass;

  export let launcherValidation: boolean;

  export let exeFileValidation: boolean;

  export let processStatus: boolean;

  const dispatch = createEventDispatcher<{
    "specify-game-path": HoyoInterface;
  }>();

  async function handleSpecifyLauncherFile() {
    const result = await hoyoClass.pickLauncherFile();
    if (result.path.length > 0 && result.file.length > 0) {
      dispatch("specify-game-path", {
        launcherPath: result.path,
        launcherFile: result.file,
        gamePath: hoyoClass.gameProp.path,
        gameFile: hoyoClass.gameProp.file,
        scriptPath: hoyoClass.scriptProp.path,
        scriptFile: hoyoClass.scriptProp.file,
      });
    }
  }

  function handleSpecifyExeFile(needCheckConfig: boolean) {
    return (event: CustomEvent<FileProp>) => {
      const exeFile = event.detail;
      const specifyExeFileParam = {
        launcherPath: hoyoClass.launcherProp.path,
        launcherFile: hoyoClass.launcherProp.file,
        gamePath: needCheckConfig ? exeFile.path : hoyoClass.gameProp.path,
        gameFile: needCheckConfig ? exeFile.file : hoyoClass.gameProp.file,
        scriptPath: exeFile.path,
        scriptFile: exeFile.file,
      };
      dispatch("specify-game-path", specifyExeFileParam);
    };
  }

  function openExplorer(path: string) {
    return async () => {
      const isOpen = await HoyoClass.openExplorerPath(path);
      if (!isOpen) {
        KMessage({
          type: "error",
          content: "文件夹打开失败",
          duration: 1000,
        });
      }
    };
  }
</script>

<div
  class="x-middle top-3 flex flex-col items-center space-y-2 w-full text-center"
>
  <h2>{gameCnName}</h2>
  {#if hoyoClass.launcherProp.path.length > 0}
    <p>已绑定{gameCnName}：</p>
    <KTooltip
      placement="bottom"
      content={`单击打开${hoyoClass.launcherProp.path}`}
    >
      <div slot="triggerEl" class="max-w-md truncate">
        <KButton
          type="success"
          size="sm"
          on:click={openExplorer(hoyoClass.launcherProp.path)}
        >
          文件夹根目录：{hoyoClass.launcherProp.path}
        </KButton>
      </div>
    </KTooltip>
    {#if exeFileValidation}
      <KTooltip
        placement="bottom"
        content={`单击打开${hoyoClass.scriptProp.path}\\${hoyoClass.scriptProp.file}`}
      >
        <p slot="triggerEl" class="max-w-md truncate">
          <KButton
            type="success"
            size="sm"
            on:click={openExplorer(hoyoClass.scriptProp.path)}
          >
            exe运行文件目录：{`${hoyoClass.scriptProp.path}\\${hoyoClass.scriptProp.file}`}
          </KButton>
        </p>
      </KTooltip>
    {:else}
      <div>
        <p class="mb-2">无效的{gameCnName}游戏目录！</p>
      </div>
    {/if}
    {#if !processStatus}
      <div class="flex flex-col items-center gap-y-2">
        <KButton type="error" size="md" on:click={handleSpecifyLauncherFile}>
          1.指定启动器目录
        </KButton>
        {#if launcherValidation}
          <AppointButton
            needCheckConfig={true}
            {hoyoClass}
            on:specify-exe={handleSpecifyExeFile(true)}
          />
          <AppointButton
            needCheckConfig={false}
            {hoyoClass}
            on:specify-exe={handleSpecifyExeFile(false)}
          />
        {/if}
      </div>
    {/if}
  {:else}
    <p>请绑定 {gameEnName} 的文件夹根目录</p>
  {/if}
</div>
{#if hoyoClass.launcherProp.path.length === 0}
  <div class="absolute-middle flex justify-center w-full">
    <KButton type="warning" size="lg" on:click={handleSpecifyLauncherFile}>
      首次使用请绑定游戏安装目录
    </KButton>
  </div>
{/if}
