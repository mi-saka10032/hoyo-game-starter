<script lang="ts" name="Header">
  import { appWindow } from "@tauri-apps/api/window";
  import Logo from '../assets/app-icon.png';
  import MinIcon from "../assets/min.svg";
  import QuitIcon from "../assets/quit.svg";

  function minimize() {
    void appWindow.minimize();
  }

  function quit() {
    void appWindow.hide();
  }
</script>

<header data-tauri-drag-region class="flex justify-between items-center pl-2 w-full h-7 bg-[#f3f3f3]">
  <left class="h-full flex items-center space-x-1">
    <img src={Logo} alt="logo" class="w-4 h-4" draggable="false" />
    <span class="text-xs leading-none">{document.title}</span>
  </left>
  <right class="h-full flex items-center">
    <button on:click={minimize}>
      <img src={MinIcon} alt="最小化" class="w-5 h-5 align-middle" draggable="false" />
    </button>
    <button on:click={quit}>
      <img src={QuitIcon} alt="退出" class="w-5 h-5 align-middle" draggable="false" />
    </button>
  </right>
</header>

<style scoped>
  button {
    padding: 0 0.5rem;
    height: 100%;
    cursor: pointer;
    border: none;

    &:hover {
      background-color: #efe7eb;
    }
  }
</style>
