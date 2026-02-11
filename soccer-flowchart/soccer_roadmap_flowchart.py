"""
Soccer Player Development Roadmap Flowchart
Visualizes the journey from beginner to professional soccer player
"""

from graphviz import Digraph


def create_soccer_roadmap():
    """Create a detailed flowchart for soccer player development"""

    # Create a new directed graph with custom styling
    dot = Digraph(comment="Soccer Player Development Roadmap", format="png")
    dot.attr(rankdir="TB", size="12,16", dpi="300")

    # Set default node and edge attributes
    dot.attr(
        "node", shape="box", style="rounded,filled", fontname="Arial", fontsize="10"
    )
    dot.attr("edge", fontname="Arial", fontsize="9")

    # Color scheme for different stages
    colors = {
        "start": "#90EE90",  # Light green
        "initial": "#87CEEB",  # Sky blue
        "developmental": "#FFD700",  # Gold
        "intermediate": "#FFA500",  # Orange
        "advanced": "#FF6347",  # Tomato
        "professional": "#9370DB",  # Medium purple
        "post_pro": "#DDA0DD",  # Plum
        "decision": "#FFE4B5",  # Moccasin
        "alternative": "#F0E68C",  # Khaki
    }

    # START NODE
    dot.node(
        "start",
        "START:\nAspiring Soccer Player",
        fillcolor=colors["start"],
        shape="ellipse",
        fontsize="12",
        style="filled",
    )

    # INITIAL STAGE
    dot.node(
        "initial1",
        "INITIAL STAGE\n\nBasic Skills Training:\n• Dribbling\n• Passing\n• Shooting",
        fillcolor=colors["initial"],
    )
    dot.node(
        "initial2",
        "Fitness Foundation:\n• Agility exercises\n• Basic conditioning\n• Coordination drills",
        fillcolor=colors["initial"],
    )

    # Decision point after initial stage
    dot.node(
        "dec1", "Continue\nTraining?", fillcolor=colors["decision"], shape="diamond"
    )
    dot.node("alt1", "Recreational\nSoccer Path", fillcolor=colors["alternative"])

    # DEVELOPMENTAL STAGE
    dot.node(
        "dev1",
        "DEVELOPMENTAL STAGE\n\nJoin Youth Team/Academy",
        fillcolor=colors["developmental"],
    )
    dot.node(
        "dev2",
        "Technical Skills:\n• Ball control\n• Positioning\n• Tactical understanding\n• Team dynamics",
        fillcolor=colors["developmental"],
    )

    # Injury checkpoint 1
    dot.node("injury1", "Injury?", fillcolor=colors["decision"], shape="diamond")
    dot.node(
        "recovery1",
        "Injury Recovery:\n• Rehabilitation\n• Physical therapy\n• Gradual return",
        fillcolor=colors["alternative"],
    )

    # INTERMEDIATE STAGE
    dot.node(
        "int1",
        "INTERMEDIATE STAGE\n\nCompetitive Youth Leagues\nRegional Clubs",
        fillcolor=colors["intermediate"],
    )
    dot.node(
        "int2",
        "Advanced Development:\n• Tactical mastery\n• Team strategy\n• Match analysis\n• Mental preparation",
        fillcolor=colors["intermediate"],
    )

    # Performance checkpoint
    dot.node(
        "perf1", "Elite\nPotential?", fillcolor=colors["decision"], shape="diamond"
    )
    dot.node("alt2", "Semi-Pro/Amateur\nCareer Path", fillcolor=colors["alternative"])

    # Injury checkpoint 2
    dot.node("injury2", "Injury?", fillcolor=colors["decision"], shape="diamond")
    dot.node(
        "recovery2",
        "Extended Recovery:\n• Medical treatment\n• Career reassessment\n• Alternative training",
        fillcolor=colors["alternative"],
    )

    # ADVANCED STAGE
    dot.node(
        "adv1",
        "ADVANCED STAGE\n\nSemi-Professional Leagues\nElite Academies",
        fillcolor=colors["advanced"],
    )
    dot.node(
        "adv2",
        "Specialized Training:\n• Position specialization\n  (Forward, Midfielder,\n   Defender, Goalkeeper)",
        fillcolor=colors["advanced"],
    )
    dot.node(
        "adv3",
        "Mental & Leadership:\n• Mental toughness\n• Leadership skills\n• Resilience building\n• Sports psychology",
        fillcolor=colors["advanced"],
    )

    # Professional readiness check
    dot.node(
        "ready1", "Professional\nReady?", fillcolor=colors["decision"], shape="diamond"
    )
    dot.node("transition1", "Consider\nCoaching Path", fillcolor=colors["alternative"])

    # PROFESSIONAL STAGE
    dot.node(
        "pro1",
        "PROFESSIONAL STAGE\n\nTrials with Pro Clubs\nNational Teams",
        fillcolor=colors["professional"],
    )
    dot.node(
        "pro2",
        "Contract Signing:\n• Professional contracts\n• Agent representation\n• Legal considerations",
        fillcolor=colors["professional"],
    )
    dot.node(
        "pro3",
        "Peak Performance:\n• Continuous fitness\n• Performance monitoring\n• Optimal diet & nutrition\n• Sleep & recovery\n• Mental health management",
        fillcolor=colors["professional"],
    )
    dot.node(
        "pro4",
        "Career Management:\n• Brand building\n• Media relations\n• Financial planning",
        fillcolor=colors["professional"],
    )

    # Career continuation check
    dot.node(
        "continue1", "Continue\nPlaying?", fillcolor=colors["decision"], shape="diamond"
    )

    # Injury checkpoint 3 (career-threatening)
    dot.node(
        "injury3",
        "Career-Ending\nInjury?",
        fillcolor=colors["decision"],
        shape="diamond",
    )
    dot.node(
        "early_retire", "Early Retirement\nDecision", fillcolor=colors["alternative"]
    )

    # POST-PROFESSIONAL STAGE
    dot.node(
        "post1",
        "POST-PROFESSIONAL STAGE\n\nRetirement from Playing",
        fillcolor=colors["post_pro"],
    )
    dot.node("post2", "Career Options:", fillcolor=colors["post_pro"], shape="box")

    # Post-career pathways
    dot.node(
        "coaching",
        "Coaching:\n• Youth coaching\n• Professional coaching\n• Technical director",
        fillcolor=colors["post_pro"],
    )
    dot.node(
        "management",
        "Management:\n• Team management\n• Sports administration\n• Club operations",
        fillcolor=colors["post_pro"],
    )
    dot.node(
        "media",
        "Media & Punditry:\n• Broadcasting\n• Analysis\n• Commentary",
        fillcolor=colors["post_pro"],
    )
    dot.node(
        "business",
        "Business Ventures:\n• Soccer academies\n• Sports equipment\n• Endorsements",
        fillcolor=colors["post_pro"],
    )
    dot.node(
        "recreational",
        "Lifelong Involvement:\n• Recreational play\n• Mentoring\n• Community soccer",
        fillcolor=colors["post_pro"],
    )

    # END NODE
    dot.node(
        "end",
        "END:\nSoccer Legacy",
        fillcolor=colors["post_pro"],
        shape="ellipse",
        fontsize="12",
        style="filled",
    )

    # EDGES - Main pathway
    dot.edge("start", "initial1")
    dot.edge("initial1", "initial2")
    dot.edge("initial2", "dec1")

    # Decision 1
    dot.edge("dec1", "dev1", label="Yes")
    dot.edge("dec1", "alt1", label="No")
    dot.edge("alt1", "recreational", style="dashed")

    # Developmental stage
    dot.edge("dev1", "dev2")
    dot.edge("dev2", "injury1")

    # Injury checkpoint 1
    dot.edge("injury1", "int1", label="No")
    dot.edge("injury1", "recovery1", label="Yes")
    dot.edge("recovery1", "dev2", label="Recovered", style="dashed")

    # Intermediate stage
    dot.edge("int1", "int2")
    dot.edge("int2", "perf1")

    # Performance checkpoint
    dot.edge("perf1", "injury2", label="Yes")
    dot.edge("perf1", "alt2", label="No")
    dot.edge("alt2", "post1", style="dashed")

    # Injury checkpoint 2
    dot.edge("injury2", "adv1", label="No")
    dot.edge("injury2", "recovery2", label="Yes")
    dot.edge("recovery2", "int2", label="Recovered", style="dashed")
    dot.edge("recovery2", "transition1", label="Career Change", style="dashed")
    dot.edge("transition1", "coaching", style="dashed")

    # Advanced stage
    dot.edge("adv1", "adv2")
    dot.edge("adv2", "adv3")
    dot.edge("adv3", "ready1")

    # Professional readiness
    dot.edge("ready1", "pro1", label="Yes")
    dot.edge("ready1", "alt2", label="Not Yet", style="dashed")

    # Professional stage
    dot.edge("pro1", "pro2")
    dot.edge("pro2", "injury3")

    # Injury checkpoint 3
    dot.edge("injury3", "pro3", label="No")
    dot.edge("injury3", "early_retire", label="Yes")
    dot.edge("early_retire", "post1", style="dashed")

    # Professional continuation
    dot.edge("pro3", "pro4")
    dot.edge("pro4", "continue1")
    dot.edge("continue1", "pro3", label="Yes", constraint="false")
    dot.edge("continue1", "post1", label="No/Retire")

    # Post-professional stage
    dot.edge("post1", "post2")
    dot.edge("post2", "coaching")
    dot.edge("post2", "management")
    dot.edge("post2", "media")
    dot.edge("post2", "business")
    dot.edge("post2", "recreational")

    # All paths lead to end
    dot.edge("coaching", "end")
    dot.edge("management", "end")
    dot.edge("media", "end")
    dot.edge("business", "end")
    dot.edge("recreational", "end")

    return dot


def main():
    """Generate and save the flowchart"""
    print("Creating Soccer Player Development Roadmap Flowchart...")

    # Create the flowchart
    flowchart = create_soccer_roadmap()

    # Save the flowchart
    output_file = "soccer_roadmap_flowchart"
    flowchart.render(output_file, cleanup=True)

    print(f"\n✓ Flowchart created successfully!")
    print(f"✓ Output file: {output_file}.png")
    print(f"\nFlowchart features:")
    print("  • Initial Stage: Basic skills and fitness")
    print("  • Developmental Stage: Youth teams and technical training")
    print("  • Intermediate Stage: Competitive leagues and tactics")
    print("  • Advanced Stage: Semi-pro and specialization")
    print("  • Professional Stage: Pro contracts and peak performance")
    print("  • Post-Professional Stage: Multiple career pathways")
    print("  • Decision Points: Continue training, injury recovery, career transitions")
    print("  • Alternative Pathways: Recreational, semi-pro, coaching transitions")

    # Also save the source DOT file for reference
    flowchart.save(f"{output_file}.dot")
    print(f"✓ Source DOT file saved: {output_file}.dot")


if __name__ == "__main__":
    main()
