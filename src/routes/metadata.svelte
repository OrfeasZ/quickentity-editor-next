<script lang="ts">
	import ListEditor from "$lib/components/ListEditor.svelte"
	import { entity, saveWorkAndCallback } from "$lib/stores"
	import { Select, TextInput } from "carbon-components-svelte"
	import { onDestroy } from "svelte"

	const unsubscribe = saveWorkAndCallback.subscribe(async (value) => {
		if (value) {
			void value()
		}
	})

	onDestroy(unsubscribe)
</script>

<div class="flex flex-col h-full p-2 px-3">
	<h1>Metadata</h1>
	<div class="mt-1 flex-grow overflow-y-auto overflow-x-hidden">
		<div class="h-full pr-2 pb-8 overflow-y-auto overflow-x-hidden">
			<div class="flex flex-wrap gap-4">
				<TextInput
					class="shepherd-factoryHash"
					labelText="Factory hash"
					bind:value={$entity.tempHash}
					style="font-family: 'Fira Code', 'IBM Plex Mono', 'Menlo', 'DejaVu Sans Mono', 'Bitstream Vera Sans Mono', Courier, monospace"
				/>
				<TextInput
					class="shepherd-blueprintHash"
					labelText="Blueprint hash"
					bind:value={$entity.tbluHash}
					style="font-family: 'Fira Code', 'IBM Plex Mono', 'Menlo', 'DejaVu Sans Mono', 'Bitstream Vera Sans Mono', Courier, monospace"
				/>
				<TextInput
					class="shepherd-rootEntity"
					labelText="Root entity"
					bind:value={$entity.rootEntity}
					style="font-family: 'Fira Code', 'IBM Plex Mono', 'Menlo', 'DejaVu Sans Mono', 'Bitstream Vera Sans Mono', Courier, monospace"
				/>
				<Select
					class="shepherd-entityType"
					labelText="Entity type"
					on:change={({ detail }) => {
						$entity.subType = detail
					}}
				>
					<option selected={$entity.subType == "template"} value="template">Template</option>
					<option selected={$entity.subType == "brick"} value="brick">Brick</option>
					<option selected={$entity.subType == "scene"} value="scene">Scene</option>
				</Select>
			</div>
			<br />
			<div class="shepherd-externalScenes">
				<span class="bx--label">External scenes</span>
				<ListEditor
					data={$entity.externalScenes}
					on:define={({ detail }) => {
						$entity.externalScenes[
							$entity.externalScenes.findIndex((a) => a == detail.original) != -1 ? $entity.externalScenes.findIndex((a) => a == detail.original) : $entity.externalScenes.length
						] = detail.new
					}}
					on:undefine={({ detail }) => ($entity.externalScenes = $entity.externalScenes.filter((a) => a != detail.value))}
				/>
				<br />
			</div>
			<div class="mt-2 grid grid-cols-2 gap-4 shepherd-extraDependencies">
				<div>
					<span class="bx--label">Extra factory dependencies</span>
					<ListEditor
						data={$entity.extraFactoryDependencies.map((a) => (typeof a == "object" ? a.resource : a))}
						on:define={({ detail }) => {
							$entity.extraFactoryDependencies[
								$entity.extraFactoryDependencies.findIndex((a) => (typeof a == "object" ? a.resource : a) == detail.original) != -1
									? $entity.extraFactoryDependencies.findIndex((a) => (typeof a == "object" ? a.resource : a) == detail.original)
									: $entity.extraFactoryDependencies.length
							] = detail.new
						}}
						on:undefine={({ detail }) => ($entity.extraFactoryDependencies = $entity.extraFactoryDependencies.filter((a) => a != detail.value))}
					/>
				</div>
				<div>
					<span class="bx--label">Extra blueprint dependencies</span>
					<ListEditor
						data={$entity.extraBlueprintDependencies.map((a) => (typeof a == "object" ? a.resource : a))}
						on:define={({ detail }) => {
							$entity.extraBlueprintDependencies[
								$entity.extraBlueprintDependencies.findIndex((a) => (typeof a == "object" ? a.resource : a) == detail.original) != -1
									? $entity.extraBlueprintDependencies.findIndex((a) => (typeof a == "object" ? a.resource : a) == detail.original)
									: $entity.extraBlueprintDependencies.length
							] = detail.new
						}}
						on:undefine={({ detail }) => ($entity.extraBlueprintDependencies = $entity.extraBlueprintDependencies.filter((a) => (typeof a == "object" ? a.resource : a) != detail.value))}
					/>
				</div>
			</div>
		</div>
	</div>
</div>
