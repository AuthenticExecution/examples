import sys
import tikzplotlib
import pandas as pd
from matplotlib import pyplot as plt
import seaborn as sns
import re
from os.path import join

file = sys.argv[1]
out_fldr = sys.argv[2]

# some set of colors for color-blind people. Just uncomment your
# favorite one (and optionally swap them)
#colors = ["#1E88E5", "#D81B60"]
colors = ["#FFC107", "#004D40"]
#colors = ["#40B0A6", "#E1BE6A"]
#colors = ["#006CD1", "#994F00"]

def save_tikz(file, width="15cm", height="8cm"):
    tikzplotlib.save(file, strict=True, axis_width=width, axis_height=height)

    with open(file, "r") as f:
        tikz_file = f.read()

    tikz_file = tikz_file.replace("% This file was created with", "% !TEX root = ../paper.tex\n% This file was created with")

    # fix legend size and spacing
    tikz_file = tikz_file.replace("legend style={", "legend style={font=\\large,")
    tikz_file = tikz_file.replace("legend style={", "legend style={/tikz/every even column/.append style={column sep=0.3cm},/tikz/every odd column/.append style={column sep=0.1cm},")
    
    # show tick labels
    tikz_file = tikz_file.replace("xmajorticks=false", "xmajorticks=true")
    tikz_file = tikz_file.replace("ymajorticks=false", "ymajorticks=true")

    tickstyle_regex = "([xy])tick style={(.+)}"
    matches = re.findall(tickstyle_regex, tikz_file)

    # hide ticks
    for match in matches:
        tikz_file = tikz_file.replace(
            f"{match[0]}tick style={{{match[1]}}}",
            f"{match[0]}tick style={{{match[1]},draw=none}}"
        )

    # set clip=false to show labels outside the plot
    tikz_file = tikz_file.replace(
        "\\begin{axis}[",
        "\\begin{axis}[\nclip=false,"
    )

    with open(file, "w") as f:
        f.write(tikz_file)

# create dataframe
df = pd.read_csv(file)
df.drop(columns=df.columns[0], axis=1, inplace=True)
df.drop(columns=["total"], axis=1, inplace=True)
df = df.groupby('size').mean().reset_index()

# from ms to s
df['sancus'] /= 1000
df['sgx'] /= 1000
df.rename(columns = {'sancus':'Sancus', 'sgx':'Intel SGX'}, inplace = True)

print(df)

# plot
sns.set_theme(style="whitegrid")
sns.set()

plt.figure(figsize=(20,20))
plot = df.plot(kind='bar', x="size", stacked=True, color=colors)

plt.ylabel("Time [s]")
plt.xlabel("Payload Size [kB]")
#plt.yticks(range(1,11))
plt.xticks(rotation=0)

sns.move_legend(
    plot, "lower center",
    bbox_to_anchor=(.5, 1), ncol=2, title=None, frameon=False,
    facecolor="white", fontsize=20
)

plt.savefig(join(out_fldr, 'tee.png'), format="png")
save_tikz(join(out_fldr, "tee.tex"))
