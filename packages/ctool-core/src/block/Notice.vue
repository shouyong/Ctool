<template>
    <div class="ctool-notice" ref="container">
        <TransitionGroup name="ctool-notice-list" tag="div" v-if="items.length > 0">
            <div class="ctool-notice-list-item" v-for="(item) in items" :key="item.key" :style="center ? {justifyContent:`center`} : {}">
                <Icon :name="item.type === 'ad' ? 'ad' : 'message'" :size="14"/>
                <template v-if="item.url.type === ''">
                    {{ item.text }}
                </template>
                <Link v-else @click="open(item)">{{ item.text }}</Link>
            </div>
        </TransitionGroup>
    </div>
</template>
<script setup lang="ts">
import {openUrl} from "@/helper/helper";
import {onMounted, onUnmounted} from "vue";
import {isString} from "lodash";
import useOperate from "@/store/operate";

const props = defineProps({
    center: {
        type: Boolean,
        default: false
    },
})

const operate = useOperate()
const container = $ref<HTMLElement | null>(null)

type ItemType = {
    key: number,
    type: "" | "info" | "ad",
    text: string,
    url: {
        type: "tool" | "web" | "",
        value: string | { tool: string, feature: string }
    },
}

let show = $ref(true);
let isToggle = $ref(true);
let items: ItemType[] = $ref<ItemType[]>([])

const CACHE_NAME = 'notice_cache_items';
const CACHE_EXPIRY = 1800;
const CACHE_EMPTY_EXPIRY = 3600 * 24;

let toggleTimer: any = null;

const init = (lists: ItemType[]) => {
    items = lists
    toggleTimer && clearInterval(toggleTimer)
    if (items.length > 1) {
        toggleTimer = setInterval(toggle, 6000);
    }
}

const toggle = () => {
    if (items.length < 2 || !isToggle) {
        return;
    }
    const item = items.splice(0, 1)
    items = [...items, item[0]]
}

const open = (item: ItemType) => {
    if (!item.url.value) {
        return;
    }
    if (item.url.type === "web") {
        if (isString(item.url.value)) {
            return openUrl(item.url.value)
        }
    }
    if (item.url.type === "tool") {
        operate.redirectTool(
            isString(item.url.value) ? item.url.value : item.url.value.tool,
            isString(item.url.value) ? "" : item.url.value.feature
        )
    }
}

const mouseover = () => {
    isToggle = false
}

const mouseleave = () => {
    isToggle = true
}

onMounted(() => {
    // load()
    container?.addEventListener('mouseleave', mouseleave)
    container?.addEventListener('mouseover', mouseover)
})

onUnmounted(() => {
    toggleTimer && clearInterval(toggleTimer)
    container?.removeEventListener('mouseleave', mouseleave)
    container?.removeEventListener('mouseover', mouseover)
})
</script>

<style scoped>
.ctool-notice {
    height: 32px;
    overflow: hidden;
}

.ctool-notice-list-item {
    height: 32px;
    font-size: 14px;
    display: flex;
    align-items: center;
    column-gap: 5px;
}

.ctool-notice-list-move,
.ctool-notice-list-enter-active,
.ctool-notice-list-leave-active {
    transition: all 0.5s ease;
}

.ctool-notice-list-enter-from,
.ctool-notice-list-leave-to {
    opacity: 0;
    transform: translateX(30px);
}

.ctool-notice-list-leave-active {
    position: absolute;
}
</style>
