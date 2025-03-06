<template>
    <HeightResize v-slot="{ small, large }" :reduce="5">
        <Align direction="vertical">
            <TextInput :allow="['text']" v-model="action.current.input" :height="small" />
            <TextOutput
                v-model="action.current.output"
                :allow="['text',  ]"
                :content="output"
                :height="large"
                @success="action.save()"
                encoding
            />
        </Align>
    </HeightResize>
</template>

<script lang="ts" setup>
import { useAction, initialize } from "@/store/action";
import { createTextInput, createTextOutput } from "@/components/text";
import LZString from 'lz-string';
import Text from "@/helper/text";

const action = useAction(
    await initialize({
        input: createTextInput("text"),
        output: createTextOutput(),
    }),
);

const output = $computed(() => {
    if (
        action.current.input.text.isEmpty()
    ) {
        return Text.empty()
    }
    if (action.current.input.text.isError()) {
        return action.current.input.text
    }
    try {

        if (!action.current.input.text.isText()) {
            throw new Error("input content must text / text file")
        }
        let result:Text = Text.fromString(LZString.decompressFromBase64(action.current.input.text.toString()))
        // result.setExtension('.gz')
        return result;
    } catch (e) {
        return Text.fromError($error(e))
    }
})
</script>
