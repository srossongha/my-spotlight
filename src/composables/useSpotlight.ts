import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { SearchResult } from "../types/spotlight";

export function useSpotlight() {
  const query = ref("");
  const results = ref<SearchResult[]>([]);
  const selectedIndex = ref(0);
  const isLoading = ref(false);

  // Cap visible results to 8 items for a neat Spotlight appearance
  const visibleResults = computed(() => results.value.slice(0, 8));
  const activeItem = computed(() => visibleResults.value[selectedIndex.value]);
  const hasResults = computed(() => visibleResults.value.length > 0);

  let searchTimeout: ReturnType<typeof setTimeout> | null = null;

  async function performSearch(searchQuery: string) {
    isLoading.value = true;
    try {
      const res = await invoke<SearchResult[]>("search", { query: searchQuery });
      results.value = res;
      selectedIndex.value = 0;
    } catch (err) {
      console.error("Search failed:", err);
      results.value = [];
    } finally {
      isLoading.value = false;
    }
  }

  function handleQueryChange(newQuery: string) {
    query.value = newQuery;
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }
    // Instant search with zero lag
    performSearch(newQuery);
  }

  function navigate(direction: "up" | "down") {
    const total = visibleResults.value.length;
    if (total === 0) return;

    if (direction === "down") {
      selectedIndex.value = (selectedIndex.value + 1) % total;
    } else {
      selectedIndex.value = (selectedIndex.value - 1 + total) % total;
    }
  }

  function selectIndex(index: number) {
    if (index >= 0 && index < visibleResults.value.length) {
      selectedIndex.value = index;
    }
  }

  async function executeCurrent() {
    const item = activeItem.value;
    if (!item) return;

    try {
      await invoke("execute_action", { action: item.action });
      // Reset search query
      query.value = "";
      performSearch("");
    } catch (err) {
      console.error("Execute action failed:", err);
    }
  }

  async function executeItem(item: SearchResult) {
    try {
      await invoke("execute_action", { action: item.action });
      query.value = "";
      performSearch("");
    } catch (err) {
      console.error("Execute action failed:", err);
    }
  }

  async function hide() {
    try {
      await invoke("hide_window");
    } catch (err) {
      console.error("Failed to hide window:", err);
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      navigate("down");
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      navigate("up");
    } else if (e.key === "Enter") {
      e.preventDefault();
      executeCurrent();
    } else if (e.key === "Escape") {
      e.preventDefault();
      hide();
    }
  }

  // Setup window event listener
  function setupEventListener() {
    listen("spotlight-shown", () => {
      query.value = "";
      performSearch("");
    });
  }

  return {
    query,
    results,
    visibleResults,
    selectedIndex,
    isLoading,
    activeItem,
    hasResults,
    handleQueryChange,
    navigate,
    selectIndex,
    executeCurrent,
    executeItem,
    hide,
    onKeyDown,
    performSearch,
    setupEventListener,
  };
}

