<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useSpotlight } from "./composables/useSpotlight";
import SearchBar from "./components/SearchBar.vue";
import ResultList from "./components/ResultList.vue";
import FooterBar from "./components/FooterBar.vue";

const {
  query,
  results,
  visibleResults,
  selectedIndex,
  activeItem,
  handleQueryChange,
  selectIndex,
  executeItem,
  onKeyDown,
  performSearch,
  setupEventListener,
} = useSpotlight();

const searchBarRef = ref<InstanceType<typeof SearchBar> | null>(null);

function handleWindowKeyDown(e: KeyboardEvent) {
  onKeyDown(e);
}

onMounted(() => {
  // Listen for global and window events
  setupEventListener();
  window.addEventListener("keydown", handleWindowKeyDown);
  // Perform initial load of default actions
  performSearch("");
  searchBarRef.value?.focusInput();
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleWindowKeyDown);
});
</script>

<template>
  <div class="w-full h-full p-2 flex flex-col justify-start select-none bg-transparent">
    <!-- Spotlight Main Floating Card -->
    <div
      class="w-full flex flex-col rounded-2xl overflow-hidden bg-neutral-900/80 backdrop-blur-3xl border border-white/15 shadow-[0_25px_60px_-15px_rgba(0,0,0,0.7)] text-white"
    >
      <!-- Search Bar -->
      <SearchBar
        ref="searchBarRef"
        :model-value="query"
        @update:model-value="handleQueryChange"
        @keydown="onKeyDown"
      />

      <!-- Search Results List -->
      <ResultList
        :results="visibleResults"
        :selected-index="selectedIndex"
        :search-query="query"
        @select="selectIndex"
        @execute="executeItem"
      />

      <!-- Bottom Hint Bar -->
      <FooterBar
        :active-item="activeItem"
        :total-results="results.length"
      />
    </div>
  </div>
</template>
