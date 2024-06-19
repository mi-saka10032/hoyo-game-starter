<script lang="ts" name="Directory">
  import { createEventDispatcher } from "svelte";
  import { KButton } from "@ikun-ui/button";
  import { KTooltip } from "@ikun-ui/tooltip";
  import AppointButton from "@/components/AppointButton.svelte";
  import { FileProp, HoyoClass, HoyoInterface } from "@/lib";

  export let gameEnName: string;

  export let gameCnName: string;

  export let hoyoClass: HoyoClass;

  export let launcherValidation: boolean;

  export let exeValidation: boolean;

  export let processStatus: boolean;

  const dispatch = createEventDispatcher<{
    "specify-game-path": HoyoInterface;
  }>();

  async function handleSpecifyLauncherFile() {
    const result = await hoyoClass.pickLauncherFile();
    if (result.path.length > 0 && result.file.length > 0) {
      dispatch("specify-game-path", {
        root: result.path,
        launcher: result.file,
        game: hoyoClass.exeProp.path,
        exe: hoyoClass.exeProp.file,
      });
    }
  }

  function handleSpecifyExeFile(event: CustomEvent<FileProp>) {
    const exeFile = event.detail;
    dispatch("specify-game-path", {
      root: hoyoClass.launcherProp.path,
      launcher: hoyoClass.launcherProp.file,
      game: exeFile.path,
      exe: exeFile.file,
    });
  }
</script>

<div
  class="x-middle top-3 flex flex-col items-center space-y-2 w-full text-center"
>
  <h2>{gameCnName}</h2>
  {#if hoyoClass.launcherProp.path.length > 0}
    <p>已绑定{gameCnName}：</p>
    <KTooltip placement="bottom" content={hoyoClass.launcherProp.path}>
      <p slot="triggerEl" class="max-w-md truncate">
        文件夹根目录：{hoyoClass.launcherProp.path}
      </p>
    </KTooltip>
    {#if exeValidation}
      <KTooltip
        placement="bottom"
        content={`${hoyoClass.exeProp.path}\\${hoyoClass.exeProp.file}`}
      >
        <p slot="triggerEl" class="max-w-md truncate">
          exe文件目录：{`${hoyoClass.exeProp.path}\\${hoyoClass.exeProp.file}`}
        </p>
      </KTooltip>
    {:else}
      <div>
        <p class="mb-2">无效的{gameCnName}游戏目录！</p>
      </div>
    {/if}
    {#if !processStatus}
      <div
        class={`flex ${
          launcherValidation ? "justify-between" : "justify-center"
        } items-center px-16 w-full`}
      >
        <KButton type="error" size="md" on:click={handleSpecifyLauncherFile}>
          指定启动器目录
        </KButton>
        {#if launcherValidation}
          <AppointButton
            needCheckConfig={true}
            {hoyoClass}
            on:specify-exe={handleSpecifyExeFile}
          />
          <AppointButton
            needCheckConfig={false}
            {hoyoClass}
            on:specify-exe={handleSpecifyExeFile}
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
