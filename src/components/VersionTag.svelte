<script lang="ts" name="VersionTag">
  import { KTag } from "@ikun-ui/tag";

  export let version: string;
  export let remoteVersion: string;
  export let hasPreDownload: boolean;
</script>

{#if version.length > 0}
  <div class="absolute top-1 left-1 flex flex-col">
    <KTag type="success">
      本地游戏版本是：{version}
    </KTag>
  </div>
{/if}
{#if remoteVersion.length > 0}
  <div class="absolute top-1 right-1 flex flex-col">
    <KTag type="success" cls="mb-2">
      实际游戏版本是：{remoteVersion}
    </KTag>
    {#if hasPreDownload}
      <KTag type="warning">可下载预下载资源包</KTag>
    {/if}
  </div>
{/if}
