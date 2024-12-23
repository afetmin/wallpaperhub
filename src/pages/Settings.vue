<template>
    <div class="h-full p-8">
        <NumberField
            class="flex items-center"
            id="lists"
            :min="0"
            v-model="value"
        >
            <Label for="lists" class="text-lg">图片</Label>:
            <NumberFieldContent>
                <NumberFieldDecrement />
                <NumberFieldInput />
                <NumberFieldIncrement />
            </NumberFieldContent>
        </NumberField>
    </div>
</template>

<script setup lang="ts">
import {
    NumberField,
    NumberFieldContent,
    NumberFieldDecrement,
    NumberFieldIncrement,
    NumberFieldInput,
} from '@/components/ui/number-field';
import { Label } from '@/components/ui/label';
import { ref } from 'vue';

const value = ref(20);
</script>
