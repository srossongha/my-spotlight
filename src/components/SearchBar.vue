<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "keydown", event: KeyboardEvent): void;
}>();

const inputRef = ref<HTMLInputElement | null>(null);

function focusInput() {
  nextTick(() => {
    inputRef.value?.focus();
  });
}

onMounted(() => {
  focusInput();
});

defineExpose({
  focusInput,
});
</script>

<template>
  <div class="relative flex items-center px-4 py-3.5 border-b border-white/10">
    <!-- Spotlight Search Icon -->
    <div class="flex items-center justify-center text-white/50 mr-3 shrink-0">
      <svg
        class="w-5 h-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2.2"
          d="M21 21l-4.35-4.35M17 11a6 6 0 11-12 0 6 6 0 0112 0z"
        />
      </svg>
    </div>

    <!-- Main Input Field -->
    <input
      ref="inputRef"
      type="text"
      :value="modelValue"
      placeholder="Spotlight Search"
      class="w-full bg-transparent text-white text-[19px] font-normal placeholder-white/40 outline-none caret-blue-400 select-auto tracking-tight"
      autofocus
      spellcheck="false"
      autocomplete="off"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      @keydown="emit('keydown', $event)"
    />

    <!-- Right Side Badges / Clear Button -->
    <div class="flex items-center space-x-2 shrink-0 ml-2">
      <button
        v-if="modelValue.length > 0"
        @click="emit('update:modelValue', ''); focusInput();"
        class="p-1 rounded-full text-white/40 hover:text-white/80 hover:bg-white/10 transition-colors"
        title="Clear search"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <div
        class="px-2 py-0.5 rounded text-[11px] font-medium tracking-wide bg-white/10 text-white/60 border border-white/5 select-none shadow-sm"
      >
        ⌥ Space
      </div>
    </div>
  </div>
</template>

