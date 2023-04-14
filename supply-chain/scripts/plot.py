import sys
import tikzplotlib
import pandas as pd
from matplotlib import pyplot as plt
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

def save_tikz(file, width="8.4cm", height="3.8cm"):
    tikzplotlib.save(file, strict=True, axis_width=width, axis_height=height)

    with open(file, "r") as f:
        tikz_file = f.read()

    tikz_file = tikz_file.replace("% This file was created with", "% !TEX root = ../paper.tex\n% This file was created with")

    # fix legend size and spacing
    tikz_file = tikz_file.replace("legend style={", "legend style={font=\\small,")
    tikz_file = tikz_file.replace("legend style={", "legend style={/tikz/every even column/.append style={column sep=0.15cm},/tikz/every odd column/.append style={column sep=0.1cm},")
    # remove weird addition of legend entries
    tikz_file = tikz_file.replace("};\n\\addlegendentry{Intel SGX}", "};")

    # show tick labels
    tikz_file = tikz_file.replace("xmajorticks=false", "xmajorticks=true")
    tikz_file = tikz_file.replace("ymajorticks=false", "ymajorticks=true")

    tickstyle_regex = "([xy])tick style={(.+)}"
    matches = re.findall(tickstyle_regex, tikz_file)
    
    # format errorbars
    tikz_file = tikz_file.replace("\\addplot [semithick, ", "\\addplot [thin, ")

    # hide ticks
    #for match in matches:
    #    tikz_file = tikz_file.replace(
    #        f"{match[0]}tick style={{{match[1]}}}",
    #        f"{match[0]}tick style={{{match[1]},draw=none}}"
    #    )

    # Changes to the output
    manual_adjustments = [
        #"clip=false", # set clip=false to show labels outside the plot
        #"yticklabel shift={-0.15cm}", # move the y tick labels closer to the plot
        #"xticklabel shift={-0.15cm}," # move the x tick labels closer to the plot
        "ylabel near ticks, ylabel shift={-0.1cm}", # move the y label closer to the plot
        "xlabel near ticks, xlabel shift={-0.05cm}", # move the x label closer to the plot
        "every major tick/.append style={major tick length=2pt}", # decrease length of tick markers
        "every tick label/.append style={font=\scriptsize}", # decrease fontsize of tick labels
        "label style={font=\\small}", # adjust label sizes
        #"legend image post style={scale=1.5}", # increase size of legend markers
    ]

    for adjust in manual_adjustments:
        tikz_file = tikz_file.replace(
            "\n]\n\\draw",
            ",\n" + adjust + "\n]\n\\draw"
        )

    with open(file, "w") as f:
        f.write(tikz_file)

# create dataframe
df = pd.read_csv(file)
df.drop(columns=df.columns[0], axis=1, inplace=True)
df.drop(columns=["total"], axis=1, inplace=True)

# from ms to s
df['sancus'] /= 1000
df['sgx'] /= 1000

dfmean = df.groupby('size').mean().reset_index()
dferr = df.groupby('size').std().reset_index()
# compute 99% CI; 
dferr['sancus'] *= 2.576
dferr['sgx'] *= 2.576

#dfmean.rename(columns = {'sancus':'Sancus', 'sgx':'Intel SGX'}, inplace = True)
ind = dfmean["size"].unique()

print(dfmean)

# Plot

cm = 1/2.54  # centimeters in inches
fig, ax = plt.subplots(figsize=(8.4*cm, 3.8*cm))
plt.tight_layout(pad=0.4, w_pad=0.5, h_pad=0.1)
ax.yaxis.grid(True)
ax.set_axisbelow(True)

error_kw = dict(elinewidth=0.25,ecolor='black',capsize=1.5, capthick=0.25)
plt.bar(ind, dfmean["sancus"], yerr=dferr["sancus"], color=colors[0], error_kw=error_kw, label='Sancus')
plt.bar(ind, dfmean["sgx"], bottom=dfmean["sancus"], yerr=dferr["sgx"], color=colors[1], error_kw=error_kw, label='Intel SGX')

plt.legend(loc="lower center", ncol=2, bbox_to_anchor=(0.5, 0.87), fontsize=7,
            edgecolor="black", framealpha=1.0, fancybox=False)

plt.ylabel("Time [s]")
plt.xlabel("Payload Size [kiB]")
plt.yticks(range(1,11))
plt.xticks(ind)

plt.savefig(join(out_fldr, 'tee.png'), format="png")
save_tikz(join(out_fldr, "tee.tex"))
