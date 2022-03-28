import { Button, Badge, Grid, Group, SimpleGrid, Tabs, Tooltip, Paper, ActionIcon, Text } from "@mantine/core";
import React from "react";
import { DeviceFloppy, Folder, Plus, X } from "tabler-icons-react";
import { SliceConfiguration } from "../components/SliceConfiguration";
import Page from "./Page";

class ConfigPage extends React.Component {
	render() {
		return (
			<Page title="Configuration">
				<Grid
					style={{
						flex: "1",
						display: "flex",
						flexDirection: "column",
						flexWrap: "nowrap",
						overflow: "auto",
					}}>
					<Grid.Col style={{ flex: "1 1 auto", display: "flex", overflow: "auto" }}>
						<Paper
							style={{ flex: "1 1 auto", display: "flex", overflow: "auto" }}
							sx={(theme) => ({
								border: `1px solid ${
									theme.colorScheme === "dark" ? theme.colors.dark[4] : theme.colors.gray[2]
								}`,
								borderRadius: `${theme.radius["md"]}px`,
							})}>
							<Tabs
								style={{
									flex: "1 1 auto",
									display: "flex",
									flexDirection: "column",
									overflow: "auto",
								}}
								styles={{
									body: { paddingTop: 0, flex: 1, display: "flex", overflow: "auto" },
								}}>
								<Tabs.Tab
									label={
										<Group spacing="xs">
											Slice 1
											<ActionIcon
												size="xs"
												sx={(theme) => ({
													color: "inherit",
													"&:hover": {
														color: theme.colors.red[4],
													},
												})}
												p={0}>
												<X />
											</ActionIcon>
										</Group>
									}
									style={{ overflow: "auto" }}>
									<SliceConfiguration></SliceConfiguration>
								</Tabs.Tab>
								<Tabs.Tab icon={<Plus size="1rem"></Plus>}></Tabs.Tab>
							</Tabs>
						</Paper>
					</Grid.Col>
					<Grid.Col style={{ flex: "0 1 0" }}>
						<SimpleGrid cols={2}>
							<Group position="left">
								<Tooltip label="Enregistrer la configuration actuelle" openDelay={1000}>
									<Button>
										<DeviceFloppy />
									</Button>
								</Tooltip>
								<Tooltip label="Ouvrir une configuration" openDelay={1000}>
									<Button>
										<Folder />
									</Button>
								</Tooltip>
							</Group>
							<Group position="right">
								<Badge size="md" variant="filled" color="green">
									Aucune erreur
								</Badge>
							</Group>
						</SimpleGrid>
					</Grid.Col>
				</Grid>
			</Page>
		);
	}
}

export default ConfigPage;
