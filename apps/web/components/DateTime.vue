<script lang="ts" setup>
import { useDays } from '~/hooks/date'

const { days, oneDayWeek, day, month, hours, minute, second,week,ymd } = useDays()

</script>

<template>
    <div class="p-8px rounded-8px shadow">
        
        <div class="flex mb-16px">
            <div class="mx-auto">{{ ymd.join('·') }}</div>
        </div>
        <div class="flex">
            <div class="flex flex-col flex-wrap gap-y-2px gap-x-12px text-[var(--text-200)] content-start h-152px">
                <span v-for="key in ['日', '一', '二', '三', '四', '五', '六']"
                    class="text-[var(--text-300)] text-12px cursor-pointer inline-flex items-center justify-center rounded-10px w-20px h-20px">
                    {{ key }}
                </span>
                <span v-for="day in oneDayWeek"
                    class="text-12px inline-flex items-center justify-center rounded-10px w-20px h-20px"></span>
                <template v-for="_day in days">
                    <span :class="_day == Number(day) ? 'ping' : ''"
                        class="text-12px cursor-pointer inline-flex items-center justify-center rounded-10px w-20px h-20px bg-[var(--bg-200)] shadow">{{
                            _day
                        }}</span>
                </template>

            </div>
            <div class="grow-1 flex flex-col items-center justify-center">
                <div class="rounded-8px overflow-hidden text-[var(--text-300)] flex items-center justify-center">
                    <TransitionGroup class="relative h-30px flex w-28px bg-[var(--bg-200)]" tag="div" name="list">
                        <span :key="hours[0] + '10'" class="absolute top-0 left-0 text-20px">{{ hours[0] }}</span>
                        <span :key="hours[1] + '01'" class="absolute top-0 left-50% text-20px">{{ hours[1]
                            }}</span>
                    </TransitionGroup>
                    ·
                    <TransitionGroup class="relative h-30px flex w-28px bg-[var(--bg-200)]" tag="div" name="list">
                        <span :key="minute[0] + '10'" class="absolute top-0 left-0 text-20px">{{ minute[0] }}</span>
                        <span :key="minute[1] + '01'" class="absolute top-0 left-50% text-20px">{{ minute[1]
                            }}</span>
                    </TransitionGroup>
                </div>
                <div>
                    {{ week }}
                </div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.ping {
    position: relative;
    z-index: 0;

    &::before {
        content: '';
        z-index: -1;
        position: absolute;
        top: 0;
        left: 0;
        background-color: var(--accent-100);
        border-radius: 50%;
        width: 20px;
        height: 20px;
        transform-origin: center;
        animation: spin 1s cubic-bezier(0, 0, 0.2, 1) infinite;
    }

    &::after {
        content: '';
        position: absolute;
        z-index: -1;
        top: 0;
        left: 0;
        background-color: var(--bg-200);
        border-radius: 50%;
        width: 20px;
        height: 20px;
        
    }
}

@keyframes spin {
    0% {
        transform: scale(1);
        opacity: 1;
    }

    75%,
    100% {
        transform: scale(2);
        opacity: 0;
    }
}

.list-enter-active {
    transition: all 0.5s 0.5s ease;
    transform: translateY(0);
}

.list-leave-active {
    transition: all 0.5s linear;
    transform: translateY(0px);
}

.list-enter-from {
    opacity: 0;
    transform: translateY(-15px);
}

.list-leave-to {
    opacity: 0;
    transform: translateY(15px);
}
</style>